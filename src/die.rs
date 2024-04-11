use crate::common::*;
use crate::probability::Probability;
use crate::probability_distribution::{ProbabilityDistribution, ProbabilityIter};
use core::ops::Add;

#[derive(Debug, Clone)]
pub struct Die {
    probabilities: Vec<Probability>,
    min: i32,
    max: i32,
    variance: f64,
    standard_deviation: f64,
    mean: f64,
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
            Die::from_values((start..=end).collect::<Vec<i32>>())
        }
    }

    pub fn from_values(values: Vec<i32>) -> Die {
        Die::from_probabilities(values_to_probabilities(&values))
    }

    pub fn from_probabilities(probabilities: Vec<Probability>) -> Self {
        if probabilities.is_empty() {
            return Die::empty();
        }
        let min = probabilities.iter().min().unwrap().value.clone();
        let max = probabilities.iter().max().unwrap().value.clone();
        let variance = calc_variance(&probabilities);
        let standard_deviation = calc_standard_deviation(&probabilities);
        let mean = calc_mean(&probabilities);
        Die {
            probabilities: compress_additive(&probabilities),
            min,
            max,
            variance,
            standard_deviation,
            mean,
        }
    }

    pub fn empty() -> Die {
        Die {
            probabilities: vec![Probability {
                value: 0,
                chance: 1.0,
            }],
            min: 0,
            max: 0,
            variance: 0 as f64,
            standard_deviation: 0 as f64,
            mean: 0 as f64,
        }
    }
}

impl ProbabilityDistribution for Die {
    fn get_probabilities(&self) -> &Vec<Probability> {
        &self.probabilities
    }

    fn get_min(&self) -> &i32 {
        &self.min
    }

    fn get_max(&self) -> &i32 {
        &self.max
    }

    fn get_mean(&self) -> &f64 {
        &self.mean
    }

    fn get_variance(&self) -> &f64 {
        &self.variance
    }

    fn get_standard_deviation(&self) -> &f64 {
        &self.standard_deviation
    }

    fn add_independent(&self, probability_distribution: &impl ProbabilityDistribution) -> Die {
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
                        .collect::<Vec<Probability>>()
                })
                .collect(),
        )
    }

    fn conditional_chain<F>(&self, callback_fn: &F) -> Die
    where
        F: Fn(&i32) -> Die,
    {
        Die::from_probabilities(compress_additive(
            &self
                .get_probabilities()
                .iter()
                .flat_map(|outer_prob| {
                    callback_fn(&outer_prob.value)
                        .get_probabilities()
                        .iter()
                        .map(|inner_prob| *inner_prob * outer_prob.chance)
                        .collect::<Vec<Probability>>()
                })
                .collect(),
        ))
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

    fn iter(&self) -> ProbabilityIter {
        ProbabilityIter::new(&self.probabilities)
    }

    fn get_results(&self) -> String {
        // TODO get rid of newline at end
        self.iter().map(|prob| format!("{prob}\n")).collect()
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
            self.min,
            "Max",
            self.max,
            "Mean",
            self.mean,
            "Variance",
            self.variance,
            "Standard Deviation",
            self.standard_deviation
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
