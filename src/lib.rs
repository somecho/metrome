#![doc = include_str!("../README.md")]

/// Metrome error types
pub mod error;
/// Scans and tokenizes a Metrome score
pub mod scanner;
/// This module contains structs that are related to the representation of a score.
pub mod score;
/// Unit conversion utilities
pub mod units;
/// Writes the click track from a score
pub mod writer;
