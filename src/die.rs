use crate::common::*;
use crate::probability::Probability;
use crate::probability_distribution::{ProbabilityDistribution, ProbabilityIter};
use core::ops::Add;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Die {
    probabilities: Vec<Probability<i32>>,
}

impl Die {
    pub fn new(sides: i32) -> Die {
        if sides == 0 {
            Die::empty()
        } else {
            Die::from_range(1, sides)
        }
    }

    pub fn from_range(start: i32, end: i32) -> Die {
        if end < start {
            panic!("Can't create a Die with the given parameters");
        }
        if end - start == 0 {
            Die::empty()
        } else {
            Die::from_values(&(start..=end).collect::<Vec<i32>>())
        }
    }

    pub fn from_values(values: &[i32]) -> Die {
        Die::from_probabilities(values_to_probabilities(values))
    }

    pub fn from_probabilities(probabilities: Vec<Probability<i32>>) -> Self {
        if probabilities.is_empty() {
            return Die::empty();
        }
        Die {
            probabilities: compress_additive(&probabilities),
        }
    }

    pub fn empty() -> Die {
        Die {
            probabilities: vec![Probability {
                value: 0,
                chance: 1.0,
            }],
        }
    }
}

impl ProbabilityDistribution<i32> for Die {
    fn get_probabilities(&self) -> &Vec<Probability<i32>> {
        &self.probabilities
    }

    fn get_min(&self) -> i32 {
        self.get_probabilities().iter().min().unwrap().value
    }

    fn get_max(&self) -> i32 {
        self.get_probabilities().iter().max().unwrap().value
    }

    fn get_mean(&self) -> f64 {
        calc_mean(self.get_probabilities())
    }

    fn get_variance(&self) -> f64 {
        calc_variance(self.get_probabilities())
    }

    fn get_standard_deviation(&self) -> f64 {
        calc_standard_deviation(self.get_probabilities())
    }

    fn add_independent(&self, probability_distribution: &impl ProbabilityDistribution<i32>) -> Die {
        Die::from_probabilities(
            probability_distribution
                .get_probabilities()
                .iter()
                .flat_map(|outer_prob| {
                    self.get_probabilities()
                        .iter()
                        .map(|inner_prob| *outer_prob + *inner_prob)
                })
                .collect(),
        )
    }

    fn add_dependent<F>(&self, callback_fn: &F) -> Die
    where
        F: Fn(&i32) -> Die,
    {
        Die::from_probabilities(
            self.get_probabilities()
                .iter()
                .flat_map(|outer_prob| {
                    callback_fn(&outer_prob.value)
                        .get_probabilities()
                        .iter()
                        .map(|inner_prob| *outer_prob + *inner_prob)
                        // dislike the collect here...
                        .collect::<Vec<Probability<i32>>>()
                })
                .collect(),
        )
    }

    fn conditional_chain<F>(&self, callback_fn: &F) -> Die
    where
        F: Fn(&i32) -> Die,
    {
        Die::from_probabilities(
            self.get_probabilities()
                .iter()
                .flat_map(|outer_prob| {
                    callback_fn(&outer_prob.value)
                        .get_probabilities()
                        .iter()
                        .map(|inner_prob| *inner_prob * outer_prob.chance)
                        .collect::<Vec<Probability<i32>>>()
                })
                .collect::<Vec<Probability<i32>>>(),
        )
    }

    fn add_flat(&self, flat_increase: i32) -> Die {
        Die::from_probabilities(
            self.get_probabilities()
                .iter()
                .map(|prob| Probability {
                    value: prob.value + flat_increase,
                    chance: prob.chance,
                })
                .collect(),
        )
    }

    fn iter(&self) -> ProbabilityIter<i32> {
        ProbabilityIter::new(&self.probabilities)
    }

    fn get_results(&self) -> String {
        // TODO get rid of newline at end
        self.iter().fold(String::new(), |mut out, prob| {
            let _ = writeln!(out, "{prob}");
            out
        })
    }

    fn get_details(&self) -> String {
        format!(
            "\
                {:<NAME_FORMAT$}{:>NUMBER_FORMAT$.DECIMAL_FORMAT$}\n\
                {:<NAME_FORMAT$}{:>NUMBER_FORMAT$.DECIMAL_FORMAT$}\n\
                {:<NAME_FORMAT$}{:>NUMBER_FORMAT$.DECIMAL_FORMAT$}\n\
                {:<NAME_FORMAT$}{:>NUMBER_FORMAT$.DECIMAL_FORMAT$}\n\
                {:<NAME_FORMAT$}{:>NUMBER_FORMAT$.DECIMAL_FORMAT$}\
                ",
            "Min",
            self.get_min(),
            "Max",
            self.get_max(),
            "Mean",
            self.get_mean(),
            "Variance",
            self.get_variance(),
            "Standard Deviation",
            self.get_standard_deviation()
        )
    }
}

impl std::fmt::Display for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_results())
    }
}

impl<'a> Add<&'a Die> for &'a Die {
    type Output = Die;

    fn add(self, rhs: &'a Die) -> Self::Output {
        self.add_independent(rhs)
    }
}

impl Add<Die> for Die {
    type Output = Die;

    fn add(self, rhs: Die) -> Self::Output {
        self.add_independent(&rhs)
    }
}

impl Add<i32> for Die {
    type Output = Die;

    fn add(self, rhs: i32) -> Self::Output {
        self.add_flat(rhs)
    }
}

impl<'a> Add<i32> for &'a Die {
    type Output = Die;

    fn add(self, rhs: i32) -> Self::Output {
        self.add_flat(rhs)
    }
}

impl<'a, F> Add<&'a F> for &'a Die
where
    F: Fn(&i32) -> Die,
{
    type Output = Die;

    fn add(self, rhs: &'a F) -> Self::Output {
        self.add_dependent(rhs)
    }
}

impl<F> Add<F> for Die
where
    F: Fn(&i32) -> Die,
{
    type Output = Die;

    fn add(self, rhs: F) -> Self::Output {
        self.add_dependent(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializers() {
        let expected_probabilities = vec![
            Probability {
                value: 1,
                chance: 0.5,
            },
            Probability {
                value: 2,
                chance: 0.5,
            },
        ];
        assert_eq!(*Die::new(2).get_probabilities(), expected_probabilities);
        assert_eq!(
            Die::from_values(&vec![1, 2]).get_probabilities(),
            &expected_probabilities
        );
        assert_eq!(
            Die::from_probabilities(expected_probabilities.clone()).get_probabilities(),
            &expected_probabilities
        );
        assert_eq!(
            Die::from_range(1, 2).get_probabilities(),
            &expected_probabilities
        );
        assert_eq!(
            *Die::empty().get_probabilities(),
            vec![Probability {
                value: 0,
                chance: 1.0
            }]
        )
    }

    #[test]
    fn mean_calculation() {
        assert_eq!(Die::new(6).get_mean(), 3.5)
    }

    #[test]
    fn variance_calculation() {
        assert_eq!(Die::new(6).get_variance(), 2.916666666666666)
    }

    #[test]
    fn standard_deviation_calculation() {
        assert_eq!(Die::new(6).get_standard_deviation(), 1.707825127659933)
    }

    #[test]
    fn min() {
        assert_eq!(
            (Die::new(2) + Die::from_values(&vec![3, 4, 5])).get_min(),
            4
        )
    }

    #[test]
    fn max() {
        assert_eq!(
            (Die::new(2) + Die::from_values(&vec![3, 4, 5])).get_max(),
            7
        )
    }

    #[test]
    fn adding() {
        assert_eq!(
            *(Die::new(2) + Die::new(2)).get_probabilities(),
            vec![
                Probability {
                    value: 2,
                    chance: 0.25
                },
                Probability {
                    value: 3,
                    chance: 0.5
                },
                Probability {
                    value: 4,
                    chance: 0.25
                },
            ]
        )
    }

    #[test]
    fn adding_dependent() {
        assert_eq!(
            *(Die::new(2) + &|&prob: &_| if prob == 2 { Die::new(2) } else { Die::new(0) })
                .get_probabilities(),
            vec![
                Probability {
                    value: 1,
                    chance: 0.5
                },
                Probability {
                    value: 3,
                    chance: 0.25
                },
                Probability {
                    value: 4,
                    chance: 0.25
                }
            ]
        );
    }

    #[test]
    fn chaining_dice() {
        assert_eq!(
            *(Die::new(2).conditional_chain(&|&prob| if prob == 1 {
                Die::new(2)
            } else {
                Die::new(3)
            }))
            .get_probabilities(),
            vec![
                Probability {
                    value: 1,
                    chance: 0.41666666666666663
                },
                Probability {
                    value: 2,
                    chance: 0.41666666666666663
                },
                Probability {
                    value: 3,
                    chance: 0.16666666666666666
                }
            ]
        )
    }

    #[test]
    fn adding_flat() {
        assert_eq!(
            *(Die::new(2) + 1).get_probabilities(),
            vec![
                Probability {
                    value: 2,
                    chance: 0.5,
                },
                Probability {
                    value: 3,
                    chance: 0.5,
                }
            ]
        )
    }
}
