use crate::common::*;
use crate::probability::Probability;
use std::fmt::Write;

/// Base structure for mutating and evaluating different types of collections of
/// [probabilities][`Probability`].
pub trait ProbabilityDistribution<T> {
    fn add_dependent<F>(&self, callback_fn: &F) -> Self
    where
        F: Fn(&T) -> Self;
    fn add_flat(&self, flat_increase: i32) -> Self;
    fn add_independent(&self, probability_distribution: &impl ProbabilityDistribution<T>) -> Self;
    fn conditional_chain<F>(&self, callback_fn: &F) -> Self
    where
        F: Fn(&T) -> Self;
    fn get_probabilities(&self) -> &Vec<Probability<T>>;

    fn get_details(&self) -> String
    where
        T: Copy + std::ops::Mul<T, Output = T> + std::fmt::Display,
        Probability<T>: Ord,
        f64: From<T>,
    {
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

    fn get_max(&self) -> T
    where
        Probability<T>: Ord,
        T: Copy,
    {
        self.get_probabilities().iter().max().unwrap().value
    }

    fn get_mean(&self) -> f64
    where
        Probability<T>: Ord,
        T: Copy + std::ops::Mul<T, Output = T>,
        f64: From<T>,
    {
        calc_mean(self.get_probabilities())
    }

    fn get_min(&self) -> T
    where
        Probability<T>: Ord,
        T: Copy,
    {
        self.get_probabilities().iter().min().unwrap().value
    }

    fn get_results(&self) -> String
    where
        Probability<T>: std::fmt::Display,
    {
        // TODO get rid of newline at end
        self.iter().fold(String::new(), |mut out, prob| {
            let _ = writeln!(out, "{prob}");
            out
        })
    }

    fn get_standard_deviation(&self) -> f64
    where
        Probability<T>: Ord,
        T: Copy + std::ops::Mul<T, Output = T>,
        f64: From<T>,
    {
        calc_standard_deviation(self.get_probabilities())
    }

    fn get_variance(&self) -> f64
    where
        Probability<T>: Ord,
        T: Copy + std::ops::Mul<T, Output = T>,
        f64: From<T>,
    {
        calc_variance(self.get_probabilities())
    }

    /// Returns an iterator over the probabilities of this distribution.
    fn iter(&self) -> ProbabilityIter<T> {
        ProbabilityIter::new(self.get_probabilities())
    }
}

/// Iterator over a list of probabilities.
pub struct ProbabilityIter<'a, T> {
    values: &'a Vec<Probability<T>>,
    index: usize,
}

impl<'a, T> ProbabilityIter<'a, T> {
    pub fn new(probabilities: &'a Vec<Probability<T>>) -> Self {
        ProbabilityIter {
            values: probabilities,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for ProbabilityIter<'a, T> {
    type Item = &'a Probability<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.values.len() {
            let result = Some(&self.values[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}
