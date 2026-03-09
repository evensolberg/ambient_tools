//! Data processing pipeline for Ambient Weather JSON files.
//!
//! Provides file reading, date filtering, and CSV export built on top of
//! the `datapoint` type system.

pub mod export;
pub mod filename;
pub mod filter;
pub mod reader;
