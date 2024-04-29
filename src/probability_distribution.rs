use crate::probability::Probability;

/// Base structure for mutating and evaluating different types of collections of
/// [probabilities][`Probability`].
pub trait ProbabilityDistribution<T> {
    fn add_independent(&self, probability_distribution: &impl ProbabilityDistribution<T>) -> Self;
    fn add_dependent<F>(&self, callback_fn: &F) -> Self
    where
        F: Fn(&T) -> Self;
    fn conditional_chain<F>(&self, callback_fn: &F) -> Self
    where
        F: Fn(&T) -> Self;
    fn add_flat(&self, flat_increase: i32) -> Self;
    fn get_probabilities(&self) -> &Vec<Probability<T>>;
    fn iter(&self) -> ProbabilityIter<T>;
    fn get_results(&self) -> String;
    fn get_details(&self) -> String;
    fn get_min(&self) -> i32;
    fn get_max(&self) -> i32;
    fn get_variance(&self) -> f64;
    fn get_standard_deviation(&self) -> f64;
    fn get_mean(&self) -> f64;
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
