use crate::probability::Probability;
use std::collections::HashMap;

pub const NAME_FORMAT: usize = 20;
pub const NUMBER_FORMAT: usize = 10;
pub const DECIMAL_FORMAT: usize = 3;
pub const BAR_LENGTH: usize = 50;

pub fn values_to_probabilities<T>(values: &[T]) -> Vec<Probability<T>>
where
    T: Copy,
{
    let chance = 1.0 / values.len() as f64;
    values
        .iter()
        .map(|value| Probability {
            value: *value,
            chance,
        })
        .collect()
}

pub fn calc_mean<T>(values: &[Probability<T>]) -> f64
where
    f64: From<T>,
    T: Copy,
{
    values
        .iter()
        .fold(0.0, |acc, prob| acc + prob.chance * f64::from(prob.value))
}

pub fn calc_variance<T>(values: &[Probability<T>]) -> f64
where
    f64: From<T>,
    T: std::ops::Mul<Output = T> + Copy,
{
    let mean = calc_mean(values);
    values.iter().fold(0.0, |acc, prob| {
        acc + prob.chance * f64::from(prob.value * prob.value)
    }) - (mean * mean)
}

pub fn calc_standard_deviation<T>(values: &[Probability<T>]) -> f64
where
    f64: From<T>,
    T: std::ops::Mul<Output = T> + Copy,
{
    calc_variance(values).sqrt()
}

pub fn compress_additive<T>(values: &[Probability<T>]) -> Vec<Probability<T>>
where
    Probability<T>: Ord,
    T: std::cmp::Eq + std::hash::Hash + Copy,
{
    let mut value_map = HashMap::new();

    for prob in values {
        if let Some(chance) = value_map.get_mut(&prob.value) {
            *chance += prob.chance;
        } else {
            value_map.insert(prob.value, prob.chance);
        }
    }

    let mut result = Vec::new();
    for (key, value) in value_map {
        result.push(Probability {
            value: key,
            chance: value,
        });
    }
    result.sort();
    result
}
