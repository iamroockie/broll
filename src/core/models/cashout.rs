#[derive(Clone, Debug, serde::Deserialize)]
pub struct Cashout {
    /// Interval hands between cashouts. Must be divisible by 100 (simulation step size)
    pub interval: u32,

    pub kind: CashoutKind,
}

impl Cashout {
    pub fn validate(&self) -> Result<(), String> {
        if self.interval == 0 || self.interval % 100 != 0 {
            return Err(format!(
                "Cashout interval ({}) must be a positive multiple of 100",
                self.interval
            ));
        }

        match self.kind {
            CashoutKind::Fixed { amount } => {
                if amount <= 0.0 {
                    return Err("Cashout amount must be positive".to_string());
                }
            }

            CashoutKind::ProfitPercentage { percentage }
            | CashoutKind::BankrollPercentage { percentage } => {
                if percentage <= 0.0 || percentage > 1.0 {
                    return Err("Cashout percentage must be between 0.0 and 1.0".to_string());
                }
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CashoutKind {
    /// Fixed amount (USD)
    Fixed { amount: f64 },

    /// Fraction of profit since last cashout (0.0 – 1.0)
    ProfitPercentage { percentage: f64 },

    /// Fraction of current bankroll (0.0 – 1.0)
    BankrollPercentage { percentage: f64 },
}
