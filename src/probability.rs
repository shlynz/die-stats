use crate::common::{BAR_LENGTH, DECIMAL_FORMAT, NUMBER_FORMAT};
use core::cmp::Ordering;
use core::ops::{Add, Mul};

/// Represents one outcome of a probability distribution.
///
/// Holds one value, and the chance for that value to appear.
///
/// # Examples
/// ```
/// # use die_stats::{ Die, Probability, ProbabilityDistribution, NormalInitializer };
/// # let coin = Die::new(2);
/// assert_eq!(
///     coin.get_probabilities(),
///     &vec![
///         Probability { value: 1, chance: 0.5 },
///         Probability { value: 2, chance: 0.5 },
///     ]);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Probability<T> {
    /// Assosiated value of this probability
    pub value: T,
    /// Odds of assosiated value happening
    pub chance: f64,
}

impl<T> Add for Probability<T>
where
    T: std::ops::Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Probability {
            value: self.value + other.value,
            chance: self.chance * other.chance,
        }
    }
}

impl<T> Mul<f64> for Probability<T> {
    type Output = Probability<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        Probability {
            value: self.value,
            chance: self.chance * rhs,
        }
    }
}

impl<T> PartialEq for Probability<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for Probability<T> where T: PartialEq {}

impl<T> PartialOrd for Probability<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Probability<T>
where
    T: Ord,
    Probability<T>: PartialOrd,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> std::fmt::Display for Probability<T>
where
    T: std::fmt::Display,
{
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
