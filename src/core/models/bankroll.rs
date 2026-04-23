#[derive(Clone, serde::Deserialize)]
pub struct BankrollInput {
    /// Starting bankroll (USD)
    pub started: f64,

    /// Bankroll below which is bust (USD)
    pub busted: f64,
}

impl BankrollInput {
    pub fn validate(&self) -> Result<(), String> {
        if self.started <= 0.0 {
            return Err("Started bankroll must be positive".into());
        }

        if self.busted < 0.0 {
            return Err("Busted bankroll cannot be negative".into());
        }

        if self.started <= self.busted {
            return Err("Started bankroll must be greater than busted bankroll".into());
        }

        Ok(())
    }
}
