use std::f64;

pub trait ProbabilityDistribution {
    fn add(&self, probability_distribution: impl ProbabilityDistribution) -> Die;
    fn add_dependent<F>(&self, callback_fn: F) -> Die
    where
        F: Fn(&i32) -> Die;
    fn get_chance(&self, value: i32) -> f64;
    fn get_result(&self) -> Vec<i32>;
    fn get_min(&self) -> i32;
    fn get_max(&self) -> i32;
    fn get_variance(&self) -> f64;
    fn get_standard_deviation(&self) -> f64;
    fn get_mean(&self) -> f64;
    fn get_median(&self) -> f64;
}

#[derive(Debug)]
pub struct Die {
    values: Vec<i32>,
    min: i32,
    max: i32,
    variance: f64,
    standard_deviation: f64,
    mean: f64,
    median: f64,
}