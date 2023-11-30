use core::ops::Add;
use std::{cmp::Ordering, f64};

pub trait ProbabilityDistribution {
    fn add_independent(&self, probability_distribution: &impl ProbabilityDistribution) -> Die;
    fn add_dependent<F>(&self, callback_fn: &F) -> Die
    where
        F: Fn(&i32) -> Die;
    fn add_flat(&self, flat_increase: i32) -> Die;
    fn get_probabilities(&self) -> &Vec<Probability>;
    fn get_min(&self) -> &i32;
    fn get_max(&self) -> &i32;
    fn get_variance(&self) -> &f64;
    fn get_standard_deviation(&self) -> &f64;
    fn get_mean(&self) -> &f64;
}

#[derive(Debug)]
pub struct Die {
    probabilities: Vec<Probability>,
    min: i32,
    max: i32,
    variance: f64,
    standard_deviation: f64,
    mean: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Probability {
    value: i32,
    chance: f64,
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
            probabilities,
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

impl Add for Probability {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Probability {
            value: self.value + other.value,
            chance: self.chance * other.chance,
        }
    }
}

impl PartialEq for Probability {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Probability {}

impl PartialOrd for Probability {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Probability {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
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

impl<'a> Add<&'a Die> for &'a Die {
    type Output = Die;

    fn add(self, rhs: &'a Die) -> Die {
        self.add_independent(rhs)
    }
}

impl<'a, F> Add<&'a F> for &'a Die
where
    F: Fn(&i32) -> Die,
{
    type Output = Die;

    fn add(self, rhs: &'a F) -> Die {
        self.add_dependent(rhs)
    }
}

fn values_to_probabilities(values: &Vec<i32>) -> Vec<Probability> {
    let chance = 1.0 / values.len() as f64;
    values
        .iter()
        .map(|value| Probability {
            value: *value,
            chance,
        })
        .collect()
}

fn calc_mean(values: &Vec<Probability>) -> f64 {
    values
        .iter()
        .fold(0.0, |acc, prob| acc + prob.chance * prob.value as f64)
}

fn calc_variance(values: &Vec<Probability>) -> f64 {
    values.iter().fold(0.0, |acc, prob| {
        acc + prob.chance * prob.value.pow(2) as f64
    }) - calc_mean(&values).powi(2)
}

fn calc_standard_deviation(values: &Vec<Probability>) -> f64 {
    calc_variance(&values).sqrt()
}
