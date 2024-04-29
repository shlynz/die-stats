//! This crate is yet another library providing (hopefully) easy to use structs for handling
//! different dice, as well as providing a foundation for implementing other sources of different
//! kinds of [probability distributions].
//!
//! Includes special cases, like [exploding] and [roll x drop n highest/lowest] and simple
//! arithmetic implementations to mutate created die.
//!
//! Yet to be implemented but planned features:
//! - [ ] Don't allow creation of a [`Die`] with total chance of >1
//! - [ ] `FromStr` to [`Die`]
//! - [ ] Complete arithmetic implementations for [`Die`]
//! - [ ] Move initializer functons from [`Die`] to own trait to keep in line with other initializer
//! traits
//! - [ ] Order functions alphabetically to keep them in line with the outline of the docs
//! - [ ] Move implementations to blanket implementations
//!   - [ ] Add for [`Probability`]
//!   - [ ] Getter for [`ProbabilityDistribution`]
//!   - [ ] Maybe even everything from the additional initializers?
//!   ([drop][`DropInitializer`]/[explode][`ExplodingInitializer`])
//! - [ ] Implement `From<i32>` for [`Die`] to convert integers to a die of just that face
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
