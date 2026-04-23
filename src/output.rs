use crate::{
    Input,
    core::{ConfidenceIntervals, ProbabilityResult, SimulationResult, SimulationStats},
};

#[derive(serde::Serialize)]
pub struct Output {
    pub elapsed_seconds: f64,
    pub num_simulations: usize,
    pub probabilities: Vec<ProbabilityResult>,
    pub ci: ConfidenceIntervals,
}

impl Output {
    pub fn new(elapsed_seconds: f64, input: &Input, results: &[SimulationResult]) -> Self {
        let num_simulations = results.len();
        let probabilities_input = input.probabilities.clone().unwrap_or(vec![]);
        let mut prob_hits = vec![0u32; probabilities_input.len()];
        let mut stats: Vec<SimulationStats> = Vec::with_capacity(num_simulations);

        for result in results {
            stats.push(result.stats.clone());

            for (pi, &hit) in result.probability_hits.iter().enumerate() {
                prob_hits[pi] += hit as u32;
            }
        }

        stats.sort_unstable_by(|b1, b2| b1.profit.total_cmp(&b2.profit));

        let inv_sims = 1.0 / num_simulations as f64;
        let probabilities = probabilities_input
            .iter()
            .enumerate()
            .map(|(idx, p)| ProbabilityResult::from_input(p, prob_hits[idx] as f64 * inv_sims))
            .collect();

        Self {
            elapsed_seconds,
            num_simulations,
            probabilities,
            ci: ConfidenceIntervals::from(stats.as_slice()),
        }
    }
}
