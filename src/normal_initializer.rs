use crate::common::values_to_probabilities;
use crate::Probability;
use core::cmp::Ordering;

/// Extended initializer for [probability distributions][`crate::ProbabilityDistribution`].
pub trait NormalInitializer<T, P: NormalInitializer<T, P>> {
    /// Creates a new distribution of type `P` from the given [`probabilities`][`Probability`].
    fn from_probabilities(probabilities: Vec<Probability<T>>) -> P;

    /// Creates an empty distribution of type `P`, meaning a singular [`probability`][`Probability`]
    /// with the equivalent of `0` as value and a chance of `1.0`.
    fn empty() -> P
    where
        T: Copy + From<i32>,
    {
        P::from_values(&[0.into()])
    }

    /// Creates a new distribution with consecutive values between, and including, start and end of
    /// type `P`. Gives every value created this way an equal amount of chance, to be specific `1/n`
    /// with `n` being the amount of values.
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

    /// Creates a new distribution of type `P` from the given values. Each value gets an equal
    /// amount of chance, but also compresses identical values to a singular
    /// [`probability`][`Probability`], to be specific `m/n` with `m` being the amount of times
    /// the given value is present and `n` being the total amount of given values.
    fn from_values(values: &[T]) -> P
    where
        T: Copy,
    {
        Self::from_probabilities(values_to_probabilities(values))
    }

    /// Creates a new distribution of type `P` from the equivalent of the first value up to, and
    /// including, the given size. Gives every value created this way an equal amount of chance, to
    /// be specific `1/n` with `n` being the amount of values in the given range.
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
