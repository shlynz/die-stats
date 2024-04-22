use crate::{Die, Probability};
use std::cmp::Ordering;

pub trait ExplodingConstructor<V, P> {
    fn new_exploding(
        sides: V,
        exploding_range: V,
        exploding_condition: Ordering,
        exploding_die: P,
    ) -> P;
    fn exploding_from_range(
        start: V,
        end: V,
        exploding_range: V,
        exploding_condition: Ordering,
        exploding_die: P,
    ) -> P;
    fn exploding_from_values(
        values: &[V],
        exploding_range: V,
        exploding_condition: Ordering,
        exploding_die: P,
    ) -> P;
    fn exploding_from_probabilities(
        probabilities: Vec<Probability<V>>,
        exploding_range: V,
        exploding_condition: Ordering,
        exploding_die: P,
    ) -> P;
}

impl ExplodingConstructor<i32, Die> for Die {
    fn new_exploding(
        sides: i32,
        exploding_range: i32,
        exploding_condition: Ordering,
        exploding_die: Die,
    ) -> Die {
        Die::new(sides) + exploding_die_helper(exploding_range, exploding_condition, exploding_die)
    }

    fn exploding_from_range(
        start: i32,
        end: i32,
        exploding_range: i32,
        exploding_condition: Ordering,
        exploding_die: Die,
    ) -> Die {
        Die::from_range(start, end)
            + exploding_die_helper(exploding_range, exploding_condition, exploding_die)
    }

    fn exploding_from_values(
        values: &[i32],
        exploding_range: i32,
        exploding_condition: Ordering,
        exploding_die: Die,
    ) -> Die {
        Die::from_values(values)
            + exploding_die_helper(exploding_range, exploding_condition, exploding_die)
    }

    fn exploding_from_probabilities(
        probabilities: Vec<Probability<i32>>,
        exploding_range: i32,
        exploding_condition: Ordering,
        exploding_die: Die,
    ) -> Die {
        Die::from_probabilities(probabilities)
            + exploding_die_helper(exploding_range, exploding_condition, exploding_die)
    }
}

fn exploding_die_helper(
    exploding_range: i32,
    exploding_condition: Ordering,
    exploding_die: Die,
) -> Box<dyn Fn(&i32) -> Die> {
    Box::new(move |&prob: &_| {
        if prob.cmp(&exploding_range) == exploding_condition {
            exploding_die.clone()
        } else {
            Die::empty()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Die;
    use std::cmp::Ordering;

    #[test]
    fn exploding_constructor() {
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
            Die::new_exploding(2, 2, Ordering::Less, Die::new(2)),
            expected_probabilities
        );
        assert_eq!(
            Die::exploding_from_values(&vec![1, 2], 2, Ordering::Less, Die::new(2)),
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
                2,
                Ordering::Less,
                Die::new(2)
            ),
            expected_probabilities
        );
        assert_eq!(
            Die::exploding_from_range(1, 2, 2, Ordering::Less, Die::new(2)),
            expected_probabilities
        );
    }
}
