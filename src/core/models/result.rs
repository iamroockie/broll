pub struct SimulationResult {
    pub stats: SimulationStats,
    pub probability_hits: Vec<bool>,
}

#[derive(Clone, Default, serde::Serialize)]
pub struct SimulationStats {
    /// Final bankroll after simulation (USD)
    pub bankroll: f64,

    /// Cashout total amount (USD)
    pub cashout: f64,

    /// Profit (USD). Calculated as `bankroll + cashout - started`.
    pub profit: f64,
}

#[derive(serde::Serialize)]
pub struct ConfidenceIntervals {
    pub p70: ConfidenceInterval,
    pub p85: ConfidenceInterval,
    pub p99: ConfidenceInterval,
}

impl From<&[SimulationStats]> for ConfidenceIntervals {
    fn from(stats: &[SimulationStats]) -> Self {
        Self {
            p70: ConfidenceInterval::from((stats, CiLevel::P70)),
            p85: ConfidenceInterval::from((stats, CiLevel::P85)),
            p99: ConfidenceInterval::from((stats, CiLevel::P99)),
        }
    }
}

pub enum CiLevel {
    P70,
    P85,
    P99,
}

impl CiLevel {
    pub const fn quantile_bounds(&self) -> (f64, f64) {
        match self {
            CiLevel::P70 => (0.15, 0.85),
            CiLevel::P85 => (0.075, 0.925),
            CiLevel::P99 => (0.005, 0.995),
        }
    }
}

#[derive(serde::Serialize)]
pub struct ConfidenceInterval {
    pub lower: SimulationStats,
    pub upper: SimulationStats,
}

impl From<(&[SimulationStats], CiLevel)> for ConfidenceInterval {
    fn from((stats, level): (&[SimulationStats], CiLevel)) -> Self {
        let (lower_q, upper_q) = level.quantile_bounds();

        let lower_idx = (lower_q * (stats.len() - 1) as f64).round() as usize;
        let upper_idx = (upper_q * (stats.len() - 1) as f64).round() as usize;

        Self {
            lower: stats[lower_idx].clone(),
            upper: stats[upper_idx].clone(),
        }
    }
}
