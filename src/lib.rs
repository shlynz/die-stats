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

impl ProbabilityDistribution for Die {
    fn add(&self, probability_distribution: impl ProbabilityDistribution) -> Die {
        unimplemented!()
    }

    fn add_dependent<F>(&self, callback_fn: F) -> Die
    where
        F: Fn(&i32) -> Die,
    {
        unimplemented!()
    }

    fn get_chance(&self, value: i32) -> f64 {
        println!("{:?}", value);
        unimplemented!()
    }

    fn get_result(&self) -> Vec<i32> {
        self.values.clone()
    }

    fn get_min(&self) -> i32 {
        self.min
    }

    fn get_max(&self) -> i32 {
        self.max
    }

    fn get_mean(&self) -> f64 {
        self.mean
    }

    fn get_median(&self) -> f64 {
        self.median
    }

    fn get_variance(&self) -> f64 {
        self.variance
    }

    fn get_standard_deviation(&self) -> f64 {
        self.standard_deviation
    }
}