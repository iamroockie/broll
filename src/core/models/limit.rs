#[derive(Clone, Debug, serde::Deserialize)]
pub struct Limit {
    pub name: LimitName,

    /// Expected value (BB/100)
    pub ev: f64,

    /// Standard deviation (BB/100)
    pub std_dev: f64,

    /// Move up to next limit when bankroll >= this value (USD). None for top limit
    pub move_up: Option<f64>,

    /// Move down to previous limit when bankroll < this value (USD). None for bottom limit
    pub move_down: Option<f64>,
}

impl Limit {
    pub fn ev_usd(&self) -> f64 {
        self.ev * self.big_blind()
    }

    pub fn std_dev_usd(&self) -> f64 {
        self.std_dev * self.big_blind()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.std_dev <= 0.0 {
            return Err(format!("Limit {self}: std dev must be positive"));
        }

        if let Some(down) = self.move_down {
            if down <= 0.0 {
                return Err(format!("Limit {self}: move down must be positive",));
            }
        }

        if let Some(up) = self.move_up {
            if up <= 0.0 {
                return Err(format!("Limit {self}: move up must be positive",));
            }
        }

        if let (Some(down), Some(up)) = (self.move_down, self.move_up) {
            if down >= up {
                return Err(format!("Limit {self}: move down must be less than move up",));
            }
        }

        Ok(())
    }

    pub const fn big_blind(&self) -> f64 {
        match self.name {
            LimitName::L2 => 0.02,
            LimitName::L5 => 0.05,
            LimitName::L10 => 0.10,
            LimitName::L25 => 0.25,
            LimitName::L50 => 0.50,
            LimitName::L100 => 1.00,
            LimitName::L200 => 2.00,
            LimitName::L500 => 5.00,
            LimitName::L1000 => 10.00,
            LimitName::L2000 => 20.00,
            LimitName::L5000 => 50.00,
            LimitName::L10000 => 100.00,
            LimitName::L20000 => 200.00,
        }
    }
}

impl std::fmt::Display for Limit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [${}]", self.name, self.big_blind())
    }
}

/// Standard poker limits, where L2 equals NL2 or PLO2 etc.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum LimitName {
    L2,
    L5,
    L10,
    L25,
    L50,
    L100,
    L200,
    L500,
    L1000,
    L2000,
    L5000,
    L10000,
    L20000,
}

impl std::fmt::Display for LimitName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
