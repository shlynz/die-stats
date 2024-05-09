use crate::{NormalInitializer, Probability, ProbabilityDistribution};

/// Used to determine the fuse.
pub enum ExplodingCondition {
    /// Explodes on everything lower than given value.
    Lower,
    /// Explodes on everything lower or equals than given value.
    LowerOrEqual,
    /// Explodes on everything equals than given value.
    Equal,
    /// Explodes on everything greater or equals than given value.
    GreaterOrEqual,
    /// Explodes on everything greater than given value.
    Greater,
}

/// Initializers for "exploding" a [probability distribution][`crate::ProbabilityDistribution`] on a given condition.
pub trait ExplodingInitializer<V, P> {
    /// Initializes a new `P` from given [probabilities][`Probability`] and explodes on given condition.
    ///
    /// Uses [`from_probabilities`][`NormalInitializer::from_probabilities`] internally.
    fn exploding_from_probabilities(
        probabilities: Vec<Probability<V>>,
        exploding_range: V,
        exploding_condition: ExplodingCondition,
        exploding: P,
    ) -> P
    where
        P: Clone + NormalInitializer<V, P> + ProbabilityDistribution<V> + 'static,
        V: Copy + Ord + From<i32> + 'static,
        i32: From<V>,
    {
        P::from_probabilities(probabilities).add_dependent(&exploding_helper(
            exploding_range,
            exploding_condition,
            exploding,
        ))
    }

    /// Initializes a new `P` from given range and explodes on given condition.
    ///
    /// Uses [`from_range`][`NormalInitializer::from_range`] internally.
    fn exploding_from_range(
        start: V,
        end: V,
        exploding_range: V,
        exploding_condition: ExplodingCondition,
        exploding: P,
    ) -> P
    where
        P: Clone + NormalInitializer<V, P> + ProbabilityDistribution<V> + 'static,
        V: Copy + Ord + From<i32> + 'static,
        i32: From<V>,
    {
        P::from_range(start, end).add_dependent(&exploding_helper(
            exploding_range,
            exploding_condition,
            exploding,
        ))
    }

    /// Initializes a new `P` from given values and explodes on given condition.
    ///
    /// Uses [`from_values`][`NormalInitializer::from_values`] internally.
    fn exploding_from_values(
        values: &[V],
        exploding_range: V,
        exploding_condition: ExplodingCondition,
        exploding: P,
    ) -> P
    where
        P: Clone + NormalInitializer<V, P> + ProbabilityDistribution<V> + 'static,
        V: Copy + Ord + From<i32> + 'static,
        i32: From<V>,
    {
        P::from_values(values).add_dependent(&exploding_helper(
            exploding_range,
            exploding_condition,
            exploding,
        ))
    }

    /// Initializes a new `P` and explodes on given condition.
    ///
    /// Uses [`new`][`NormalInitializer::new`] internally.
    fn new_exploding(
        amount: V,
        exploding_range: V,
        exploding_condition: ExplodingCondition,
        exploding: P,
    ) -> P
    where
        P: Clone + NormalInitializer<V, P> + ProbabilityDistribution<V> + 'static,
        V: Copy + Ord + From<i32> + 'static,
        i32: From<V>,
    {
        P::new(amount).add_dependent(&exploding_helper(
            exploding_range,
            exploding_condition,
            exploding,
        ))
    }
}

impl<V, P> ExplodingInitializer<V, P> for P
where
    P: Clone + NormalInitializer<V, P> + ProbabilityDistribution<V> + 'static,
    V: Copy + Ord + From<i32> + 'static,
    i32: From<V>,
{
}

fn exploding_helper<V, P>(
    exploding_range: V,
    exploding_condition: ExplodingCondition,
    exploding: P,
) -> Box<dyn Fn(&V) -> P>
where
    P: Clone + NormalInitializer<V, P> + 'static,
    V: Copy + Ord + From<i32> + 'static,
{
    Box::new(move |&prob: &_| {
        if match exploding_condition {
            ExplodingCondition::Lower => prob < exploding_range,
            ExplodingCondition::LowerOrEqual => prob <= exploding_range,
            ExplodingCondition::Equal => prob == exploding_range,
            ExplodingCondition::GreaterOrEqual => prob >= exploding_range,
            ExplodingCondition::Greater => prob > exploding_range,
        } {
            exploding.clone()
        } else {
            P::empty()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Die;

    #[test]
    fn exploding_condition_equality() {
        let expected_die = Die::new(3);
        let lower_fn = exploding_helper(0, ExplodingCondition::Lower, expected_die.clone());
        assert_eq!(lower_fn(&-1), expected_die.clone());
        assert_ne!(lower_fn(&0), expected_die.clone());
        assert_ne!(lower_fn(&1), expected_die.clone());
        let lower_eq_fn =
            exploding_helper(0, ExplodingCondition::LowerOrEqual, expected_die.clone());
        assert_eq!(lower_eq_fn(&-1), expected_die.clone());
        assert_eq!(lower_eq_fn(&0), expected_die.clone());
        assert_ne!(lower_eq_fn(&1), expected_die.clone());
        let eq_fn = exploding_helper(0, ExplodingCondition::Equal, expected_die.clone());
        assert_ne!(eq_fn(&-1), expected_die.clone());
        assert_eq!(eq_fn(&0), expected_die.clone());
        assert_ne!(eq_fn(&1), expected_die.clone());
        let greater_eq_fn =
            exploding_helper(0, ExplodingCondition::GreaterOrEqual, expected_die.clone());
        assert_ne!(greater_eq_fn(&-1), expected_die.clone());
        assert_eq!(greater_eq_fn(&0), expected_die.clone());
        assert_eq!(greater_eq_fn(&1), expected_die.clone());
        let greater_fn = exploding_helper(0, ExplodingCondition::Greater, expected_die.clone());
        assert_ne!(greater_fn(&-1), expected_die.clone());
        assert_ne!(greater_fn(&0), expected_die.clone());
        assert_eq!(greater_fn(&1), expected_die.clone());
    }

    #[test]
    fn exploding_initializer() {
        let expected_probabilities = Die::from_probabilities(vec![
            Probability {
                value: 2,
                chance: 0.75,
            },
            Probability {
                value: 3,
                chance: 0.25,
            },
        ]);
        assert_eq!(
            Die::new_exploding(2, 1, ExplodingCondition::LowerOrEqual, Die::new(2)),
            expected_probabilities
        );
        assert_eq!(
            Die::exploding_from_values(
                &vec![1, 2],
                1,
                ExplodingCondition::LowerOrEqual,
                Die::new(2)
            ),
            expected_probabilities
        );
        assert_eq!(
            Die::exploding_from_probabilities(
                vec![
                    Probability {
                        value: 1,
                        chance: 0.5,
                    },
                    Probability {
                        value: 2,
                        chance: 0.5,
                    }
                ],
                1,
                ExplodingCondition::LowerOrEqual,
                Die::new(2)
            ),
            expected_probabilities
        );
        assert_eq!(
            Die::exploding_from_range(1, 2, 1, ExplodingCondition::LowerOrEqual, Die::new(2)),
            expected_probabilities
        );
    }
}
