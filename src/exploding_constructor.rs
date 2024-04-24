use crate::{Die, Probability};

pub enum ExplodingCondition {
    Lower,
    LowerOrEqual,
    Equal,
    GreaterOrEqual,
    Greater,
}

pub trait ExplodingInitializer<V, P> {
    fn new_exploding(
        amount: V,
        exploding_range: V,
        exploding_condition: ExplodingCondition,
        exploding_die: P,
    ) -> P;
    fn exploding_from_range(
        start: V,
        end: V,
        exploding_range: V,
        exploding_condition: ExplodingCondition,
        exploding_die: P,
    ) -> P;
    fn exploding_from_values(
        values: &[V],
        exploding_range: V,
        exploding_condition: ExplodingCondition,
        exploding_die: P,
    ) -> P;
    fn exploding_from_probabilities(
        probabilities: Vec<Probability<V>>,
        exploding_range: V,
        exploding_condition: ExplodingCondition,
        exploding_die: P,
    ) -> P;
}

impl ExplodingInitializer<i32, Die> for Die {
    fn new_exploding(
        sides: i32,
        exploding_range: i32,
        exploding_condition: ExplodingCondition,
        exploding_die: Die,
    ) -> Die {
        Die::new(sides) + exploding_die_helper(exploding_range, exploding_condition, exploding_die)
    }

    fn exploding_from_range(
        start: i32,
        end: i32,
        exploding_range: i32,
        exploding_condition: ExplodingCondition,
        exploding_die: Die,
    ) -> Die {
        Die::from_range(start, end)
            + exploding_die_helper(exploding_range, exploding_condition, exploding_die)
    }

    fn exploding_from_values(
        values: &[i32],
        exploding_range: i32,
        exploding_condition: ExplodingCondition,
        exploding_die: Die,
    ) -> Die {
        Die::from_values(values)
            + exploding_die_helper(exploding_range, exploding_condition, exploding_die)
    }

    fn exploding_from_probabilities(
        probabilities: Vec<Probability<i32>>,
        exploding_range: i32,
        exploding_condition: ExplodingCondition,
        exploding_die: Die,
    ) -> Die {
        Die::from_probabilities(probabilities)
            + exploding_die_helper(exploding_range, exploding_condition, exploding_die)
    }
}

fn exploding_die_helper(
    exploding_range: i32,
    exploding_condition: ExplodingCondition,
    exploding_die: Die,
) -> Box<dyn Fn(&i32) -> Die> {
    Box::new(move |&prob: &_| {
        if match exploding_condition {
            ExplodingCondition::Lower => prob < exploding_range,
            ExplodingCondition::LowerOrEqual => prob <= exploding_range,
            ExplodingCondition::Equal => prob == exploding_range,
            ExplodingCondition::GreaterOrEqual => prob >= exploding_range,
            ExplodingCondition::Greater => prob > exploding_range,
        } {
            exploding_die.clone()
        } else {
            Die::empty()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploding_condition_equality() {
        let expected_die = Die::new(3);
        let lower_fn = exploding_die_helper(0, ExplodingCondition::Lower, expected_die.clone());
        assert_eq!(lower_fn(&-1), expected_die.clone());
        assert_ne!(lower_fn(&0), expected_die.clone());
        assert_ne!(lower_fn(&1), expected_die.clone());
        let lower_eq_fn =
            exploding_die_helper(0, ExplodingCondition::LowerOrEqual, expected_die.clone());
        assert_eq!(lower_eq_fn(&-1), expected_die.clone());
        assert_eq!(lower_eq_fn(&0), expected_die.clone());
        assert_ne!(lower_eq_fn(&1), expected_die.clone());
        let eq_fn = exploding_die_helper(0, ExplodingCondition::Equal, expected_die.clone());
        assert_ne!(eq_fn(&-1), expected_die.clone());
        assert_eq!(eq_fn(&0), expected_die.clone());
        assert_ne!(eq_fn(&1), expected_die.clone());
        let greater_eq_fn =
            exploding_die_helper(0, ExplodingCondition::GreaterOrEqual, expected_die.clone());
        assert_ne!(greater_eq_fn(&-1), expected_die.clone());
        assert_eq!(greater_eq_fn(&0), expected_die.clone());
        assert_eq!(greater_eq_fn(&1), expected_die.clone());
        let greater_fn = exploding_die_helper(0, ExplodingCondition::Greater, expected_die.clone());
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
