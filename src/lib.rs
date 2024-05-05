//! This crate is yet another library providing (hopefully) easy to use structs for handling
//! different dice, as well as providing a foundation for implementing other sources of different
//! kinds of [probability distributions].
//!
//! Includes special cases, like [exploding] and [roll x drop n highest/lowest] and simple
//! arithmetic implementations to mutate created die.
//!
//! Yet to be implemented but planned features:
//! - [ ] `FromStr` to [`Die`]
//! - [ ] Complete arithmetic implementations for [`Die`]
//! - [ ] Order functions alphabetically to keep them in line with the outline of the docs
//! - [ ] Round results from getters to avoid floating point imprecisions
//!
//! [probability distributions]: `ProbabilityDistribution`
//! [exploding]: `ExplodingInitializer`
//! [roll x drop n highest/lowest]: `DropInitializer`

pub use crate::{
    die::Die,
    drop_initializer::{DropInitializer, DropType},
    exploding_initializer::{ExplodingCondition, ExplodingInitializer},
    normal_initializer::NormalInitializer,
    probability::Probability,
    probability_distribution::{ProbabilityDistribution, ProbabilityIter},
};

mod common;
mod die;
mod drop_initializer;
mod exploding_initializer;
mod normal_initializer;
mod probability;
mod probability_distribution;
