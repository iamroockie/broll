use crate::core::LimitName;

#[derive(Clone, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProbabilityInput {
    /// Probability of going bust (bankroll <= busted) at any point during the simulation
    Bust,

    /// Probability of reaching (bankroll + cashout - starting_bankroll) >= amount at end of simulation
    Profit { amount: f64 },

    /// Probability of reaching cashout >= amount at end of simulation
    Cashout { amount: f64 },

    /// Probability of reaching bankroll >= amount at end of simulation
    Bankroll { amount: f64 },

    /// Probability of reaching a limit >= limit_name at end of simulation
    ReachLimit { limit_name: LimitName },
}

#[derive(serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProbabilityResult {
    Bust {
        probability: f64,
    },
    Profit {
        amount: f64,
        probability: f64,
    },
    Cashout {
        amount: f64,
        probability: f64,
    },
    Bankroll {
        amount: f64,
        probability: f64,
    },
    ReachLimit {
        limit_name: LimitName,
        probability: f64,
    },
}

impl ProbabilityResult {
    pub fn from_input(input: &ProbabilityInput, probability: f64) -> Self {
        match input {
            ProbabilityInput::Bust => Self::Bust { probability },
            ProbabilityInput::Profit { amount } => Self::Profit {
                amount: *amount,
                probability,
            },
            ProbabilityInput::Cashout { amount } => Self::Cashout {
                amount: *amount,
                probability,
            },
            ProbabilityInput::Bankroll { amount } => Self::Bankroll {
                amount: *amount,
                probability,
            },
            ProbabilityInput::ReachLimit { limit_name } => Self::ReachLimit {
                limit_name: limit_name.clone(),
                probability,
            },
        }
    }
}
