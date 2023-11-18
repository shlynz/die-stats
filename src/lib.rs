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

impl Die {
    pub fn new(sides: i32) -> Die {
        Die::from_range(1, sides)
    }

    fn from_range(start: i32, end: i32) -> Die {
        Die::from_values((start..=end).collect::<Vec<i32>>())
    }

    fn from_values(values: Vec<i32>) -> Die {
        if values.is_empty() {
            panic!("Contains no values")
        };
        let min = values.iter().min().unwrap().clone();
        let max = values.iter().max().unwrap().clone();
        let variance = calc_variance(&values);
        let standard_deviation = calc_standard_deviation(&values);
        let mean = calc_mean(&values);
        let median = calc_median(&values);
        Die {
            values,
            min,
            max,
            variance,
            standard_deviation,
            mean,
            median,
        }
    }
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

fn calc_mean(values: &Vec<i32>) -> f64 {
    let sum = values.iter().sum::<i32>() as f64;
    let count = values.len();
    return if count > 0 { sum / count as f64 } else { 0.0 };
}

fn calc_variance(values: &Vec<i32>) -> f64 {
    let mean = calc_mean(&values);
    values
        .iter()
        .map(|value| {
            let diff = mean - (*value as f64);
            diff * diff
        })
        .sum::<f64>()
        / values.len() as f64
}

fn calc_standard_deviation(values: &Vec<i32>) -> f64 {
    calc_variance(&values).sqrt()
}

fn calc_median(values: &Vec<i32>) -> f64 {
    let mut sorted_values = values.clone();
    sorted_values.sort();
    let sorted_values = sorted_values
        .into_iter()
        .map(|value| f64::from(value))
        .collect::<Vec<f64>>();
    let index_middle = sorted_values.len() / 2 - 1;

    return if sorted_values.len() == 1 {
        sorted_values[0]
    } else if sorted_values.len() % 2 == 0 {
        sorted_values[index_middle] + sorted_values[index_middle + 1] / 2 as f64
    } else {
        sorted_values[index_middle]
    };
}