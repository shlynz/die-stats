use crate::common::{BAR_LENGTH, DECIMAL_FORMAT, NUMBER_FORMAT};
use core::cmp::Ordering;

use core::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Probability<T> {
    pub value: T,
    pub chance: f64,
}

impl Add for Probability<i32> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Probability {
            value: self.value + other.value,
            chance: self.chance * other.chance,
        }
    }
}

impl Mul<f64> for Probability<i32> {
    type Output = Probability<i32>;

    fn mul(self, rhs: f64) -> Self::Output {
        Probability {
            value: self.value,
            chance: self.chance * rhs,
        }
    }
}

impl PartialEq for Probability<i32> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Probability<i32> {}

impl PartialOrd for Probability<i32> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Probability<i32> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl std::fmt::Display for Probability<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>NUMBER_FORMAT$} : {:>NUMBER_FORMAT$.DECIMAL_FORMAT$} : {:-<BAR_LENGTH$}",
            self.value,
            self.chance * 100.0,
            "#".repeat((self.chance * BAR_LENGTH as f64).floor() as usize)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding() {
        assert_eq!(
            Probability {
                value: 1,
                chance: 0.2
            } + Probability {
                value: 2,
                chance: 0.05
            },
            Probability {
                value: 3,
                chance: 0.01
            }
        )
    }

    #[test]
    fn multiplying() {
        assert_eq!(
            Probability {
                value: 1,
                chance: 0.2
            } * 0.05,
            Probability {
                value: 1,
                chance: 0.01
            }
        )
    }
}
