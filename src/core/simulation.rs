use rand::{Rng, RngExt as _, SeedableRng as _, rngs::SmallRng};
use rand_distr::Normal;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    Input,
    core::{Cashout, CashoutKind, Limit, ProbabilityInput, SimulationResult, SimulationStats},
};

pub fn run_simulations(
    input: &Input,
    seed: Option<u64>,
    num_simulations: usize,
) -> Vec<SimulationResult> {
    let distrs = compute_distributions(&input.limits);

    (0..num_simulations)
        .into_par_iter()
        .map(|idx| {
            let mut rng = match seed {
                Some(s) => SmallRng::seed_from_u64(s ^ (idx as u64)),
                None => rand::make_rng(),
            };
            run_simulation(input, &distrs, &mut rng)
        })
        .collect()
}

fn run_simulation<R: Rng>(input: &Input, distrs: &[Normal<f64>], rng: &mut R) -> SimulationResult {
    let mut is_bust = false;
    let mut hands: u32 = 0;
    let mut bankroll = input.bankroll.started;
    let mut cashout = 0.0;
    let mut current_limit_idx = select_limit_idx(bankroll, 0, &input.limits);
    let mut last_cashout_bankroll = input.bankroll.started;

    while hands < input.hands && !is_bust {
        hands += 100;
        let sample: f64 = rng.sample(distrs[current_limit_idx]);
        bankroll += sample;

        if let Some(c) = input.cashout.as_ref()
            && hands % c.interval == 0
        {
            let withdrawal = compute_withdrawal(c, bankroll, last_cashout_bankroll);
            if withdrawal > 0.0 {
                cashout += withdrawal;
                bankroll -= withdrawal;
                last_cashout_bankroll = bankroll;
            }
        }

        if bankroll < input.bankroll.busted {
            is_bust = true;
            break;
        }

        current_limit_idx = select_limit_idx(bankroll, current_limit_idx, &input.limits);
    }

    let profit = bankroll + cashout - input.bankroll.started;

    let probabilities_input = input.probabilities.clone().unwrap_or(vec![]);
    let mut probability_hits = Vec::with_capacity(probabilities_input.len());
    for p in probabilities_input.iter() {
        probability_hits.push(match p {
            ProbabilityInput::Bust => is_bust,
            ProbabilityInput::Profit { amount } => profit >= *amount,
            ProbabilityInput::Bankroll { amount } => bankroll >= *amount,
            ProbabilityInput::Cashout { amount } => cashout >= *amount,
            ProbabilityInput::ReachLimit { limit_name } => {
                let limit_idx = input
                    .limits
                    .iter()
                    .position(|l| &l.name == limit_name)
                    .unwrap_or(0);
                current_limit_idx >= limit_idx
            }
        });
    }

    SimulationResult {
        stats: SimulationStats {
            bankroll,
            profit,
            cashout,
        },
        probability_hits,
    }
}

fn compute_distributions(limits: &[Limit]) -> Vec<Normal<f64>> {
    limits
        .iter()
        .map(|l| Normal::new(l.ev_usd(), l.std_dev_usd()).unwrap())
        .collect()
}

fn compute_withdrawal(cashout: &Cashout, bankroll: f64, last_cashout_bankroll: f64) -> f64 {
    let withdrawal = match cashout.kind {
        CashoutKind::Fixed { amount } => amount,
        CashoutKind::ProfitPercentage { percentage } => {
            let profit = bankroll - last_cashout_bankroll;
            if profit <= 0.0 {
                return 0.0;
            }
            profit * percentage
        }
        CashoutKind::BankrollPercentage { percentage } => bankroll * percentage,
    };

    withdrawal.min(bankroll).max(0.0)
}

fn select_limit_idx(bankroll: f64, current_idx: usize, limits: &[Limit]) -> usize {
    let mut idx = current_idx;
    let limits_num = limits.len();

    // Move up as far as possible
    while idx + 1 < limits_num {
        match limits[idx].move_up {
            Some(t) if bankroll >= t => idx += 1,
            _ => break,
        }
    }

    // Move down as far as necessary
    while idx > 0 {
        match limits[idx].move_down {
            Some(t) if bankroll < t => idx -= 1,
            _ => break,
        }
    }

    idx
}
