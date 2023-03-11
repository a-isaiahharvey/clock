//! This crate provides the core logic for the clock apps implemented in various platforms.

#![warn(missing_docs, clippy::pedantic, clippy::cargo, clippy::all)]

/// The Stopwatch module provides a stopwatch for timing code execution or
/// other events. It allows you to start, stop, and reset the stopwatch, as
/// well as record lap times.
pub mod stopwatch;

/// macOS bindings
#[cfg(target_os = "macos")]
pub mod macos;
