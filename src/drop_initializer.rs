use crate::{Die, Probability, ProbabilityDistribution};

/// Used to determine what to drop.
pub enum DropType {
    /// Used to start dropping from the highest.
    High,
    /// Used to start dropping from the lowest.
    Low,
}

/// Initializers for dropping `n` results from the evaluated pool of [probability
/// distributions][`ProbabilityDistribution`].
pub trait DropInitializer<V, P> {
    /// Initializes a new `P` and drops `roll_amount` from the specified end.
    fn new_drop(amount: V, roll_amount: usize, drop_amount: V, drop_condition: DropType) -> P;
    /// Initializes a new `P` from a given range and drops `roll_amount` from the specified end.
    fn drop_from_range(
        start: V,
        end: V,
        roll_amount: usize,
        drop_amount: V,
        drop_condition: DropType,
    ) -> P;
    /// Initializes a new `P` from given values and drops `roll_amount` from the specified end.
    fn drop_from_values(
        values: &[V],
        roll_amount: usize,
        drop_amount: V,
        drop_condition: DropType,
    ) -> P;
    /// Initializes a new `P` from given [probabilities][`Probability`] and drops `roll_amount` from the specified end.
    fn drop_from_probabilities(
        probabilities: Vec<Probability<V>>,
        roll_amount: usize,
        drop_amount: V,
        drop_condition: DropType,
    ) -> P;
}

fn prep_dice(dice: &[Die]) -> Vec<(Vec<i32>, f64)> {
    if let Some(first) = dice.first() {
        let first: Vec<Vec<Probability<i32>>> = first
            .get_probabilities()
            .iter()
            .map(|val| vec![*val])
            .collect();
        dice[1..]
            .iter()
            .fold(first, |acc, curr| {
                acc.iter()
                    .flat_map(|prev_val| {
                        curr.get_probabilities()
                            .iter()
                            .map(|val_to_add| {
                                let mut new_v1 = prev_val.clone();
                                new_v1.push(*val_to_add);
                                new_v1
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .iter()
            .map(|combination| {
                let (value, chance) =
                    combination
                        .iter()
                        .fold((vec![], 1.0), |(mut values, chance), curr| {
                            values.push(curr.value);
                            let chance = chance * curr.chance;
                            (values, chance)
                        });
                (value, chance)
            })
            .collect()
    } else {
        Vec::new()
    }
}

fn drop_by_condition(dice: &[Die], drop_condition: DropType, drop_amount: i32) -> Die {
    Die::from_probabilities(
        prep_dice(dice)
            .iter()
            .map(|(values, chance)| {
                let mut new_values = values.clone();
                new_values.sort();

                match drop_condition {
                    DropType::High => (),
                    DropType::Low => new_values.reverse(),
                }

                for _ in 0..drop_amount {
                    new_values.pop();
                }

                let value: i32 = new_values.iter().sum();

                Probability {
                    value,
                    chance: *chance,
                }
            })
            .collect(),
    )
}

impl DropInitializer<i32, Die> for Die {
    fn new_drop(sides: i32, roll_amount: usize, drop_amount: i32, drop_condition: DropType) -> Die {
        drop_by_condition(
            &vec![Die::new(sides); roll_amount],
            drop_condition,
            drop_amount,
        )
    }

    fn drop_from_range(
        start: i32,
        end: i32,
        roll_amount: usize,
        drop_amount: i32,
        drop_condition: DropType,
    ) -> Die {
        drop_by_condition(
            &vec![Die::from_range(start, end); roll_amount],
            drop_condition,
            drop_amount,
        )
    }

    fn drop_from_values(
        values: &[i32],
        roll_amount: usize,
        drop_amount: i32,
        drop_condition: DropType,
    ) -> Die {
        drop_by_condition(
            &vec![Die::from_values(values); roll_amount],
            drop_condition,
            drop_amount,
        )
    }

    fn drop_from_probabilities(
        probabilities: Vec<Probability<i32>>,
        roll_amount: usize,
        drop_amount: i32,
        drop_condition: DropType,
    ) -> Die {
        drop_by_condition(
            &vec![Die::from_probabilities(probabilities); roll_amount],
            drop_condition,
            drop_amount,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prep_dice_same() {
        let input = vec![Die::new(2), Die::new(2), Die::new(2)];
        let fn_result = prep_dice(&input);
        assert_eq!(
            fn_result,
            vec![
                (vec![1, 1, 1], 0.125),
                (vec![1, 1, 2], 0.125),
                (vec![1, 2, 1], 0.125),
                (vec![1, 2, 2], 0.125),
                (vec![2, 1, 1], 0.125),
                (vec![2, 1, 2], 0.125),
                (vec![2, 2, 1], 0.125),
                (vec![2, 2, 2], 0.125),
            ]
        )
    }

    #[test]
    fn prep_dice_difference() {
        let input = vec![Die::new(2), Die::new(3), Die::new(1)];
        let fn_result = prep_dice(&input);
        assert_eq!(
            fn_result,
            vec![
                (vec![1, 1, 1], 0.16666666666666666),
                (vec![1, 2, 1], 0.16666666666666666),
                (vec![1, 3, 1], 0.16666666666666666),
                (vec![2, 1, 1], 0.16666666666666666),
                (vec![2, 2, 1], 0.16666666666666666),
                (vec![2, 3, 1], 0.16666666666666666),
            ]
        )
    }

    #[test]
    fn drop_by_condition_low() {
        assert_eq!(
            drop_by_condition(
                &vec![Die::new(2), Die::new(2), Die::new(2)],
                DropType::Low,
                1
            )
            .get_probabilities(),
            &vec![
                Probability {
                    value: 2,
                    chance: 0.5
                },
                Probability {
                    value: 3,
                    chance: 0.375
                },
                Probability {
                    value: 4,
                    chance: 0.125
                },
            ]
        );
    }

    #[test]
    fn drop_by_condition_high() {
        assert_eq!(
            drop_by_condition(
                &vec![Die::new(2), Die::new(2), Die::new(2)],
                DropType::Low,
                1
            )
            .get_probabilities(),
            &vec![
                Probability {
                    value: 2,
                    chance: 0.125
                },
                Probability {
                    value: 3,
                    chance: 0.375
                },
                Probability {
                    value: 4,
                    chance: 0.5
                },
            ]
        );
    }

    #[test]
    fn drop_initializers() {
        let expected_output = Die::from_probabilities(vec![
            Probability {
                value: 2,
                chance: 0.012345679012345678,
            },
            Probability {
                value: 3,
                chance: 0.04938271604938271,
            },
            Probability {
                value: 4,
                chance: 0.18518518518518517,
            },
            Probability {
                value: 5,
                chance: 0.345679012345679,
            },
            Probability {
                value: 6,
                chance: 0.4074074074074074,
            },
        ]);
        assert_eq!(Die::new_drop(3, 4, 2, DropType::Low), expected_output);
        assert_eq!(
            Die::drop_from_range(1, 3, 4, 2, DropType::Low),
            expected_output
        );
        assert_eq!(
            Die::drop_from_values(&vec![1, 2, 3], 4, 2, DropType::Low),
            expected_output
        );
        assert_eq!(
            Die::drop_from_probabilities(
                vec![
                    Probability {
                        value: 1,
                        chance: 0.3333333333333333
                    },
                    Probability {
                        value: 2,
                        chance: 0.3333333333333333
                    },
                    Probability {
                        value: 3,
                        chance: 0.3333333333333333
                    }
                ],
                4,
                2,
                DropType::Low
            ),
            expected_output
        )
    }
}