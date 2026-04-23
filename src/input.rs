use crate::core::{BankrollInput, Cashout, Limit, ProbabilityInput};

#[derive(Clone, serde::Deserialize)]
pub struct Input {
    pub hands: u32,
    pub bankroll: BankrollInput,
    pub limits: Vec<Limit>,
    pub cashout: Option<Cashout>,
    pub probabilities: Option<Vec<ProbabilityInput>>,
}

impl Input {
    pub fn validate(&self) -> Result<(), String> {
        if self.hands == 0 || self.hands % 100 != 0 {
            return Err("Hands must be a positive multiple of 100".into());
        }

        self.bankroll.validate()?;

        self.validate_limits()?;

        if let Some(c) = self.cashout.as_ref() {
            c.validate()?;
        }

        Ok(())
    }

    fn validate_limits(&self) -> Result<(), String> {
        if self.limits.is_empty() {
            return Err("At least one limit is required".to_string());
        }

        self.validate_limits_order()?;

        for (idx, limit) in self.limits.iter().enumerate() {
            limit.validate()?;

            if idx > 0 && limit.move_down.is_none() {
                return Err(format!("Limit {limit}: move down is required",));
            }

            if idx + 1 == self.limits.len() {
                break;
            }

            if limit.move_up.is_none() {
                return Err(format!("Limit {limit}: move up is required",));
            }

            let next = &self.limits[idx + 1];

            if let (Some(up), Some(up_next)) = (limit.move_up, next.move_up) {
                if up >= up_next {
                    return Err(format!(
                        "Move up of {limit} must be less than move up of {next}"
                    ));
                }
            }

            if let (Some(up), Some(down_next)) = (limit.move_up, next.move_down) {
                if up <= down_next {
                    return Err(format!(
                        "Move up of {limit} must be greater than move down of {next}",
                    ));
                }
            }
        }

        Ok(())
    }

    fn validate_limits_order(&self) -> Result<(), String> {
        for i in 1..self.limits.len() {
            let limit = &self.limits[i - 1];
            let next = &self.limits[i];
            if limit.big_blind() >= next.big_blind() {
                return Err(format!(
                    "Limits must be in ascending order: {limit} >= {next}",
                ));
            }
        }

        Ok(())
    }
}
