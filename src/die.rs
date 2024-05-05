use crate::common::*;
use crate::probability::Probability;
use crate::probability_distribution::ProbabilityDistribution;
use crate::NormalInitializer;
use core::ops::Add;

/// A representation of a die, using the provided initializers.
///
/// Can provide various stats via the implemented [probability distribution][`ProbabilityDistribution`] trait
/// and is already implementing both other special initializing traits, [exploding][`crate::ExplodingInitializer`]
/// and [roll x keep n][`crate::DropInitializer`].
///
/// # Examples
/// ```
/// # use die_stats::{ Die, Probability, ProbabilityDistribution, NormalInitializer };
/// let d2 = Die::new(2);
/// assert_eq!(
///     d2.get_probabilities(),
///     &vec![
///         Probability { value: 1, chance: 0.5 },
///         Probability { value: 2, chance: 0.5 }
///     ]);
/// ```
///
/// # Examples: Special/Empty die
/// ```
/// # use die_stats::{ Die, Probability, ProbabilityDistribution, NormalInitializer };
/// let d0 = Die::empty();
/// assert_eq!(
///     d0.get_probabilities(),
///     &vec![ Probability { value: 0, chance: 1.0 } ]
/// );
/// ```
///
/// # Examples: Other initializers
/// Sometimes another form of die is needed, that for example doesn't start at 1 or isn't
/// continuous.
/// ```
/// # use die_stats::{ Die, Probability, ProbabilityDistribution, NormalInitializer };
/// let d2 = Die::from_range(1, 2);
/// assert_eq!(d2, Die::new(2));
///
/// let d2 = Die::from_values(&vec![1, 2]);
/// assert_eq!(d2, Die::new(2));
///
/// let probabilities = vec![
///     Probability { value: 1, chance: 0.5 },
///     Probability { value: 2, chance: 0.5 },
/// ];
/// let d2 = Die::from_probabilities(probabilities);
/// assert_eq!(d2, Die::new(2));
/// ```
#[derive(Debug, Clone)]
pub struct Die {
    probabilities: Vec<Probability<i32>>,
}

impl NormalInitializer<i32, Die> for Die {
    /// Creates a new die with the given [probabilities][`Probability<i32>`].
    ///
    /// When given `0`, creates an [empty die][`Die::empty()`].
    ///
    /// # Examples
    /// ```
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution, NormalInitializer };
    /// let weighted_d4 = Die::from_probabilities(
    ///     vec![
    ///         Probability { value: 1, chance: 0.1 },
    ///         Probability { value: 2, chance: 0.1 },
    ///         Probability { value: 3, chance: 0.1 },
    ///         Probability { value: 4, chance: 0.7 },
    ///     ]
    /// );
    ///
    /// assert_eq!(
    ///     weighted_d4,
    ///     Die::from_values(&vec![1,2,3,4,4,4,4,4,4,4])
    /// );
    /// ```
    fn from_probabilities(probabilities: Vec<Probability<i32>>) -> Die {
        if probabilities.is_empty() {
            return Die::empty();
        }
        Die {
            probabilities: compress_additive(&probabilities),
        }
    }
}

impl ProbabilityDistribution<i32> for Die {
    fn get_probabilities(&self) -> &Vec<Probability<i32>> {
        &self.probabilities
    }

    /// Add an independent die to this one.
    ///
    /// Creates and returns a new die as a result.
    ///
    /// # Examples
    /// ```
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution, NormalInitializer };
    /// let two_d6 = Die::new(6).add_independent(&Die::new(6));
    /// assert_eq!(
    ///     two_d6.get_mean(),
    ///     7.0
    /// );
    /// ```
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

    /// Add a dependent die to this one.
    ///
    /// Creates and returns a new die as a result.
    ///
    /// # Examples
    /// ```
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution, NormalInitializer };
    /// let d4_double_if_max = Die::new(4).add_dependent(&|&val| {
    ///     if val == 4 {
    ///         Die::new(4)
    ///     } else {
    ///         Die::empty()
    ///     }
    /// });
    /// assert_eq!(
    ///     d4_double_if_max.get_probabilities(),
    ///     &vec![
    ///         Probability { value: 1, chance: 0.25 },
    ///         Probability { value: 2, chance: 0.25 },
    ///         Probability { value: 3, chance: 0.25 },
    ///         Probability { value: 5, chance: 0.0625 },
    ///         Probability { value: 6, chance: 0.0625 },
    ///         Probability { value: 7, chance: 0.0625 },
    ///         Probability { value: 8, chance: 0.0625 },
    ///     ]
    /// );
    /// ```
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

    /// Add an independent die to this one.
    ///
    /// # Examples
    /// ```
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution, NormalInitializer };
    /// let hit_or_miss = Die::new(20).conditional_chain(&|&val| {
    ///     if val >= 16 {
    ///         Die::new(1)
    ///     } else {
    ///         Die::empty()
    ///     }
    /// });
    /// assert_eq!(
    ///     hit_or_miss.get_probabilities(),
    ///     &vec![
    ///         Probability { value: 0, chance: 0.75 },
    ///         Probability { value: 1, chance: 0.25 },
    ///     ]);
    /// ```
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

    /// Adds a flat amount to a die.
    ///
    /// # Examples
    /// ```
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution, NormalInitializer };
    /// let d4_plus_two = Die::new(4).add_flat(2);
    /// assert_eq!(
    ///     d4_plus_two.get_probabilities(),
    ///     &vec![
    ///         Probability { value: 3, chance: 0.25 },
    ///         Probability { value: 4, chance: 0.25 },
    ///         Probability { value: 5, chance: 0.25 },
    ///         Probability { value: 6, chance: 0.25 },
    ///     ]
    /// );
    /// ```
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

impl PartialEq for Die {
    fn eq(&self, other: &Self) -> bool {
        self.get_probabilities() == other.get_probabilities()
    }
}

impl Eq for Die {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NormalInitializer;

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
        let expected_die = Die::from_probabilities(expected_probabilities.clone());
        // baseline test for other initializers
        assert_eq!(expected_die.get_probabilities(), &expected_probabilities);
        // other initializers
        assert_eq!(Die::new(2), expected_die);
        assert_eq!(Die::from_values(&vec![1, 2]), expected_die);
        assert_eq!(Die::from_range(1, 2), expected_die);
        assert_eq!(
            Die::empty(),
            Die::from_probabilities(vec![Probability {
                value: 0,
                chance: 1.0
            }])
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
