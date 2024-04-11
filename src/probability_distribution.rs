use crate::{die::Die, probability::Probability};

pub trait ProbabilityDistribution {
    fn add_independent(&self, probability_distribution: &impl ProbabilityDistribution) -> Die;
    fn add_dependent<F>(&self, callback_fn: &F) -> Die
    where
        F: Fn(&i32) -> Die;
    fn conditional_chain<F>(&self, callback_fn: &F) -> Die
    where
        F: Fn(&i32) -> Die;
    fn add_flat(&self, flat_increase: i32) -> Die;
    fn get_probabilities(&self) -> &Vec<Probability>;
    fn iter(&self) -> ProbabilityIter;
    fn get_results(&self) -> String;
    fn get_details(&self) -> String;
    fn get_min(&self) -> &i32;
    fn get_max(&self) -> &i32;
    fn get_variance(&self) -> &f64;
    fn get_standard_deviation(&self) -> &f64;
    fn get_mean(&self) -> &f64;
}

pub struct ProbabilityIter<'a> {
    values: &'a Vec<Probability>,
    index: usize,
}

impl<'a> ProbabilityIter<'a> {
    pub fn new(probabilities: &'a Vec<Probability>) -> Self {
        ProbabilityIter {
            values: probabilities,
            index: 0,
        }
    }
}

impl<'a> Iterator for ProbabilityIter<'a> {
    type Item = &'a Probability;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.index < self.values.len() {
            let result = Some(&self.values[self.index]);
            self.index += 1;
            result
        } else {
            None
        };
    }
}
