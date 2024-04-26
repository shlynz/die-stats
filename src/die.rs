use crate::common::*;
use crate::probability::Probability;
use crate::probability_distribution::{ProbabilityDistribution, ProbabilityIter};
use core::ops::Add;
use std::fmt::Write;

/// A representation of a die, using the provided initializers.
///
/// Can provide various stats via the implemented [probability distribution][`ProbabilityDistribution`] trait
/// and is already implementing both other special initializing traits, [exploding][`crate::ExplodingInitializer`]
/// and [roll x keep n][`crate::DropInitializer`].
///
/// # Examples
/// ```
/// # use die_stats::{ Die, Probability, ProbabilityDistribution };
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
/// # use die_stats::{ Die, Probability, ProbabilityDistribution };
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
/// # use die_stats::{ Die, Probability, ProbabilityDistribution };
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

impl Die {
    /// Creates a new die with the given amount of sides, starting from 1 and probabilities are evenly
    /// distributed.
    ///
    /// When given `0`, creates an [empty die][`Die::empty`].
    ///
    /// # Errors
    /// TODO
    ///
    /// # Example
    /// ```
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution };
    /// let d4 = Die::new(4);
    /// assert_eq!(
    ///     d4.get_probabilities(),
    ///     &vec![
    ///         Probability { value: 1, chance: 0.25 },
    ///         Probability { value: 2, chance: 0.25 },
    ///         Probability { value: 3, chance: 0.25 },
    ///         Probability { value: 4, chance: 0.25 },
    ///     ]);
    /// ```
    pub fn new(sides: i32) -> Die {
        // TODO make it save to take negative numbers
        // generate them, but negative; -3 => die of -1, -2, -3
        if sides == 0 {
            Die::empty()
        } else {
            Die::from_range(1, sides)
        }
    }

    /// Creates a new die with continuous values between and including start to end, probabilities
    /// are evenly distributed.
    ///
    /// When given `0`, creates an [empty die][`Die::empty()`].
    ///
    /// # Examples
    /// ```
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution };
    /// let d4_minus_2 = Die::from_range(-1, 2);
    /// assert_eq!(
    ///     d4_minus_2.get_probabilities(),
    ///     &vec![
    ///         Probability { value: -1, chance: 0.25 },
    ///         Probability { value:  0, chance: 0.25 },
    ///         Probability { value:  1, chance: 0.25 },
    ///         Probability { value:  2, chance: 0.25 },
    ///     ]);
    ///
    /// let d6 = Die::from_range(1, 6);
    /// assert_eq!(
    ///     d6,
    ///     Die::new(6)
    /// );
    /// ```
    pub fn from_range(start: i32, end: i32) -> Die {
        match end.cmp(&start) {
            std::cmp::Ordering::Less => Die::from_range(end, start),
            _ => Die::from_values(&(start..=end).collect::<Vec<i32>>()),
        }
    }

    /// Creates a new die from the given values, probabilities are evenly distributed.
    /// Compresses values if given duplicates.
    ///
    /// When given `0`, creates an [empty die][`Die::empty()`].
    ///
    /// # Examples
    /// ```
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution };
    /// let d4 = Die::from_values(&vec![1,2,3,4]);
    /// assert_eq!(
    ///     d4.get_probabilities(),
    ///     &vec![
    ///         Probability { value: 1, chance: 0.25 },
    ///         Probability { value: 2, chance: 0.25 },
    ///         Probability { value: 3, chance: 0.25 },
    ///         Probability { value: 4, chance: 0.25 },
    ///     ]
    /// );
    ///
    /// let skewed_die = Die::from_values(&vec![1,2,4,4]);
    /// assert_eq!(
    ///     skewed_die.get_probabilities(),
    ///     &vec![
    ///         Probability { value: 1, chance: 0.25 },
    ///         Probability { value: 2, chance: 0.25 },
    ///         Probability { value: 4, chance: 0.5 },
    ///     ]
    /// );
    /// ```
    pub fn from_values(values: &[i32]) -> Die {
        Die::from_probabilities(values_to_probabilities(values))
    }

    /// Creates a new die with the given [probabilities][`Probability<i32>`].
    ///
    /// When given `0`, creates an [empty die][`Die::empty()`].
    ///
    /// # Examples
    /// ```
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution };
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
    pub fn from_probabilities(probabilities: Vec<Probability<i32>>) -> Self {
        if probabilities.is_empty() {
            return Die::empty();
        }
        Die {
            probabilities: compress_additive(&probabilities),
        }
    }

    /// Creates a new, special, empty die with only one value, `0` with a chance of `1.0`.
    ///
    /// Can be used to return nothing for specific values when adding or chaining conditionally.
    ///
    /// # Examples
    /// ```
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution };
    /// assert_eq!(
    ///     Die::empty().get_probabilities(),
    ///     &vec![Probability { value: 0, chance: 1.0 }]
    /// );
    /// ```
    /// # Example: Use case getting average damage
    /// Calculating the average damage of a level 1 rogue with 16 dex, attacking a target with AC of 16, using a dagger.
    /// ```
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution };
    /// let proficiency_bonus = 2;
    /// let dex_modifier = 3;
    /// let attack = Die::new(20) + proficiency_bonus + dex_modifier;
    /// let armor_class = 16;
    /// let damage = attack.conditional_chain(&|&attack_result| {
    ///     if attack_result >= armor_class {
    ///         // damage of a dagger
    ///         Die::new(4)
    ///     } else {
    ///         Die::empty()
    ///     }
    /// });
    /// let average_damage = damage.get_mean();
    /// // needs to be truncated because of floating point imprecision
    /// assert_eq!(
    ///     format!("{average_damage:.2}"),
    ///     "1.25");
    /// ```
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

    /// Add an independent die to this one.
    ///
    /// Creates and returns a new die as a result.
    ///
    /// # Examples
    /// ```
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution };
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
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution };
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
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution };
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
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution };
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

    /// Returns an iterator over the probabilities of this die.
    ///
    /// # Examples
    /// ```
    /// # use die_stats::{ Die, Probability, ProbabilityDistribution };
    /// let sum_of_d6_faces = Die::new(6).iter().map(|prob| prob.value).sum::<i32>();
    /// assert_eq!(
    ///     sum_of_d6_faces,
    ///     21
    /// );
    /// ```
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

impl PartialEq for Die {
    fn eq(&self, other: &Self) -> bool {
        self.get_probabilities() == other.get_probabilities()
    }
}

impl Eq for Die {}

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
