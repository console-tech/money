use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    cents: i64, 
}

impl Money {
    // Construtor:  f64, i64 or &str
    pub fn new<T>(value: T) -> Self where T: Into<MoneyValue>, {
        let cents = match value.into() {
            MoneyValue::Float(f) => (f * 100.0).round() as i64,
            MoneyValue::Int(i) => i * 100,
        };
        Money { cents }
    }

    pub fn add(&self, other: Money) -> Money {
        Money {
            cents: self.cents + other.cents,
        }
    }

    pub fn subtract(&self, other: Money) -> Money {
        Money {
            cents: self.cents - other.cents,
        }
    }


    pub fn multiply(&self, factor: f64) -> Money {
        Money {
            cents: ((self.cents as f64) * factor).round() as i64,
        }
    }

    pub fn divide(&self, divisor: f64) -> Money {
        Money {
            cents: ((self.cents as f64) / divisor).round() as i64,
        }
    }

    pub fn get_cents(&self) -> i64 {
        self.cents
    }

    pub fn get_value(&self) -> f64 {
        self.cents as f64 / 100.0
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "R${:.2}", self.get_value())
    }
}

// Aux convert using enum
pub enum MoneyValue {
    Float(f64),
    Int(i64),
}

impl From<f64> for MoneyValue {
    fn from(v: f64) -> Self {
        MoneyValue::Float(v)
    }
}

impl From<i64> for MoneyValue {
    fn from(v: i64) -> Self {
        MoneyValue::Int(v)
    }
}

impl From<&str> for MoneyValue {
    fn from(s: &str) -> Self {
        MoneyValue::Float(s.parse::<f64>().unwrap_or(0.0))
    }
}