//! Tzolkin library for Dreamspell calendar calculations
//!
//! This library provides core functionality for calculating Mayan/Dreamspell calendar
//! correspondences based on birth dates.

pub use error::*;
pub use tables::*;

// Re-export both APIs for different use cases
pub use calculator::{kin, archetype, kin_from_parts, Tzolkin, Language};
pub use db::Seal;

pub mod calculator;
pub mod db;
pub mod error;
pub mod tables;