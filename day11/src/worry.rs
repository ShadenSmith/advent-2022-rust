use std::ops::{Add, Div, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Worry(pub i64);

impl Add for Worry {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Worry(self.0 + other.0)
    }
}

impl Mul for Worry {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Worry(self.0 * other.0)
    }
}

impl Div for Worry {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Worry(self.0 / other.0)
    }
}

impl Worry {
    pub fn parse(token: &str) -> Self {
        Worry(token.parse::<i64>().expect("Invalid value."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worry_add() {
        assert_eq!(Worry(42), Worry(32) + Worry(10));
    }

    #[test]
    fn worry_mul() {
        assert_eq!(Worry(42), Worry(6) * Worry(7));
    }
}
