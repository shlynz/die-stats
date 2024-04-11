use crate::probability::Probability;
use std::collections::HashMap;

pub const NAME_FORMAT: usize = 20;
pub const NUMBER_FORMAT: usize = 10;
pub const DECIMAL_FORMAT: usize = 3;
pub const BAR_LENGTH: usize = 50;

pub fn values_to_probabilities(values: &Vec<i32>) -> Vec<Probability> {
    let chance = 1.0 / values.len() as f64;
    values
        .iter()
        .map(|value| Probability {
            value: *value,
            chance,
        })
        .collect()
}

pub fn calc_mean(values: &Vec<Probability>) -> f64 {
    values
        .iter()
        .fold(0.0, |acc, prob| acc + prob.chance * prob.value as f64)
}

pub fn calc_variance(values: &Vec<Probability>) -> f64 {
    values.iter().fold(0.0, |acc, prob| {
        acc + prob.chance * prob.value.pow(2) as f64
    }) - calc_mean(&values).powi(2)
}

pub fn calc_standard_deviation(values: &Vec<Probability>) -> f64 {
    calc_variance(&values).sqrt()
}

pub fn compress_additive(values: &Vec<Probability>) -> Vec<Probability> {
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
