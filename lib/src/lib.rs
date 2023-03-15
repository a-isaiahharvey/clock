//! This crate provides the core logic for the clock apps implemented in various platforms.

#![warn(missing_docs, clippy::pedantic, clippy::cargo, clippy::all)]

pub mod stopwatch;
pub mod timer;

#[cfg(target_os = "macos")]
pub mod macos;
