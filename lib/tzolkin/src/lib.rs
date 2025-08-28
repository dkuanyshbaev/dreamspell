//! Tzolkin library for Dreamspell calendar calculations
//!
//! This library provides core functionality for calculating Mayan/Dreamspell calendar
//! correspondences based on birth dates.

pub use db::{get_all_seals, get_seal, Seal};
pub use tzolkin::{archetype, kin, kin_from_parts, Language, Tzolkin};

pub mod db;
mod tables;
mod tzolkin;
