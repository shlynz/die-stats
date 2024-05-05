use crate::common::values_to_probabilities;
use crate::Probability;
use core::cmp::Ordering;

/// Extended initializer for [probability distributions][`crate::ProbabilityDistribution`].
pub trait NormalInitializer<T, P: NormalInitializer<T, P>> {
    /// required
    fn from_probabilities(probabilities: Vec<Probability<T>>) -> P;

    /// provided
    fn empty() -> P
    where
        T: Copy + From<i32>,
    {
        P::from_values(&vec![0.into()])
    }

    fn from_values(values: &[T]) -> P
    where
        T: Copy,
    {
        Self::from_probabilities(values_to_probabilities(values))
    }

    fn from_range(start: T, end: T) -> P
    where
        T: Copy + Ord + From<i32>,
        i32: From<T>,
    {
        match end.cmp(&start) {
            std::cmp::Ordering::Less => Self::from_range(end, start),
            _ => {
                let converted_start: i32 = start.into();
                let converted_end: i32 = end.into();
                Self::from_values(
                    &(converted_start..=converted_end)
                        .map(|val| val.into())
                        .collect::<Vec<T>>(),
                )
            }
        }
    }

    fn new(size: T) -> P
    where
        T: Copy + Ord + From<i32>,
        i32: From<T>,
    {
        match size.cmp(&0.into()) {
            Ordering::Less => Self::from_range(size, (-1).into()),
            Ordering::Equal => Self::empty(),
            Ordering::Greater => Self::from_range(1.into(), size),
        }
    }
}
