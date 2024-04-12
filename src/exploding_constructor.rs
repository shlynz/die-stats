use super::*;

pub trait ExplodingConstructor<V, P> {
    fn new_exploding(sides: V, exploding_range: V, exploding_die: P) -> P;
    fn exploding_from_range(start: V, end: V, exploding_range: V, exploding_die: P) -> P;
    fn exploding_from_values(values: &[V], exploding_range: V, exploding_die: P) -> P;
    fn exploding_from_probabilities(
        probabilities: Vec<Probability<V>>,
        exploding_range: V,
        exploding_die: P,
    ) -> P;
}

impl ExplodingConstructor<i32, Die> for Die {
    fn new_exploding(sides: i32, exploding_range: i32, exploding_die: Die) -> Die {
        Die::new(sides) + exploding_die_helper(exploding_range, exploding_die)
    }

    fn exploding_from_range(start: i32, end: i32, exploding_range: i32, exploding_die: Die) -> Die {
        Die::from_range(start, end) + exploding_die_helper(exploding_range, exploding_die)
    }

    fn exploding_from_values(values: &[i32], exploding_range: i32, exploding_die: Die) -> Die {
        Die::from_values(values) + exploding_die_helper(exploding_range, exploding_die)
    }

    fn exploding_from_probabilities(
        probabilities: Vec<Probability<i32>>,
        exploding_range: i32,
        exploding_die: Die,
    ) -> Die {
        Die::from_probabilities(probabilities)
            + exploding_die_helper(exploding_range, exploding_die)
    }
}

fn exploding_die_helper(exploding_range: i32, exploding_die: Die) -> Box<dyn Fn(&i32) -> Die> {
    Box::new(move |&prob: &_| {
        if prob <= exploding_range {
            exploding_die.clone()
        } else {
            Die::empty()
        }
    })
}

// TODO tests
