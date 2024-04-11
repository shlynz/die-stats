use crate::common::*;
use crate::probability::Probability;
use crate::probability_distribution::{ProbabilityDistribution, ProbabilityIter};
use core::ops::Add;

#[derive(Debug, Clone)]
pub struct Die {
    probabilities: Vec<Probability>,
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

impl ProbabilityDistribution for Die {
    fn get_probabilities(&self) -> &Vec<Probability> {
        &self.probabilities
    }

    fn get_min(&self) -> i32 {
        self.get_probabilities().iter().min().unwrap().value
    }

    fn get_max(&self) -> i32 {
        self.get_probabilities().iter().max().unwrap().value
    }

    fn get_mean(&self) -> f64 {
        calc_mean(&self.get_probabilities())
    }

    fn get_variance(&self) -> f64 {
        calc_variance(&self.get_probabilities())
    }

    fn get_standard_deviation(&self) -> f64 {
        calc_standard_deviation(&self.get_probabilities())
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
