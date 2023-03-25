//! The Stopwatch module provides a stopwatch for timing code execution or
//! other events. It allows you to start, stop, and reset the stopwatch, as
//! well as record lap times.

use std::time::{Duration, SystemTime};

/// A struct representing a lap time, which includes the lap number and the time it took to complete the lap.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LapTime {
    lap_number: usize,
    split_time: Duration,
    total_time: Duration,
}

impl LapTime {
    /// Creates a new instance of a `LapTime` struct
    ///
    /// # Arguments
    /// * `lap_number` - A `u32` representing the lap number.
    /// * `split_time` - A `Duration` representing the split time.
    /// * `total_time` - A `Duration` representing the total lap time.
    ///
    /// # Returns
    /// A `LapTime` struct containing the provided lap data.
    #[must_use]
    pub fn new(lap_number: usize, split_time: Duration, total_time: Duration) -> Self {
        Self {
            lap_number,
            split_time,
            total_time,
        }
    }

    /// A `u32` representing the lap number.
    #[must_use]
    pub fn lap_number(&self) -> usize {
        self.lap_number
    }

    /// A `Duration` representing the time it took to complete the lap from the last split.
    #[must_use]
    pub fn split_time(&self) -> Duration {
        self.split_time
    }

    /// A `Duration` representing the time it took to complete the lap from the start of the stopwatch.
    #[must_use]
    pub fn total_time(&self) -> Duration {
        self.total_time
    }
}

/// A stopwatch that can be used to measure elapsed time and lap times.
///
/// The `Stopwatch` struct can be used to measure elapsed time as well as lap times. It can be started, paused,
/// and reset as needed, and it keeps track of all lap times that have been recorded.
#[repr(C)]
#[derive(Debug)]
pub struct Stopwatch {
    /// A boolean that is true if the stopwatch is currently running and false if it is not.
    running: bool,
    /// An `SystemTime` representing the time when the stopwatch was last started or reset.
    total_start_time: SystemTime,
    /// An `SystemTime` representing the time when the last lap was started.
    split_start_time: SystemTime,
    /// A `Duration` representing the elapsed time since the stopwatch was last started or reset.
    elapsed_time: Duration,
    /// An `Option<SystemTime>` representing the time when the stopwatch was last paused or None if it has never been paused.
    paused_time: Option<SystemTime>,
    /// A `Vec<LapTime>` representing the lap times for the stopwatch.
    lap_times: Vec<LapTime>,
}

impl Stopwatch {
    /// A constructor that creates a new `Stopwatch` with default values.
    #[must_use]
    pub fn new() -> Self {
        Self {
            running: false,
            total_start_time: SystemTime::now(),
            split_start_time: SystemTime::now(),
            elapsed_time: Duration::from_secs(0),
            paused_time: None,
            lap_times: Vec::new(),
        }
    }

    /// Starts the stopwatch. If the stopwatch is already running, this method does nothing.
    pub fn start(&mut self) {
        if !self.running {
            if let Some(paused_time) = self.paused_time {
                self.total_start_time +=
                    if let Ok(duration) = SystemTime::now().duration_since(paused_time) {
                        duration
                    } else {
                        self.reset();
                        return;
                    };
                self.split_start_time +=
                    if let Ok(duration) = SystemTime::now().duration_since(paused_time) {
                        duration
                    } else {
                        self.reset();
                        return;
                    };
            } else {
                self.total_start_time = SystemTime::now();
                self.split_start_time = SystemTime::now();
            }
            self.running = true;
            self.paused_time = None;
        }
    }

    /// Stops the stopwatch. If the stopwatch is not running, this method does nothing.
    pub fn stop(&mut self) {
        if self.running {
            self.paused_time = Some(SystemTime::now());
            self.elapsed_time = if let Ok(duration) = self.total_start_time.elapsed() {
                duration
            } else {
                self.reset();
                return;
            };
            self.running = false;
        }
    }

    /// Resets the stopwatch to its initial state.
    pub fn reset(&mut self) {
        self.running = false;
        self.paused_time = None;
        self.total_start_time = SystemTime::now();
        self.elapsed_time = Duration::from_secs(0);
        self.lap_times.clear();
    }

    /// Adds a lap time to the stopwatch.
    pub fn add_lap(&mut self) {
        let now = SystemTime::now();
        if self.running {
            let lap_time = LapTime {
                lap_number: self.lap_times.len() + 1,
                split_time: if let Ok(duration) = now.duration_since(self.split_start_time) {
                    duration
                } else {
                    self.reset();
                    return;
                },
                total_time: if let Ok(duration) = self.total_start_time.elapsed() {
                    duration
                } else {
                    self.reset();
                    return;
                },
            };

            self.lap_times.push(lap_time);
            self.elapsed_time = if let Ok(duration) = self.total_start_time.elapsed() {
                duration
            } else {
                self.reset();
                return;
            };
        }

        self.split_start_time = now;
    }

    /// Returns true if the stopwatch is currently running, and false otherwise.
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Returns the elapsed time since the stopwatch was last started or reset.
    pub fn elapsed_time(&mut self) -> Duration {
        let mut elapsed = if self.paused_time.is_none() {
            Duration::from_secs(0)
        } else if let Ok(duration) = self.total_start_time.elapsed() {
            duration
        } else {
            self.reset();
            return self.elapsed_time;
        };

        if self.is_running() {
            elapsed = if let Ok(duration) = self.total_start_time.elapsed() {
                duration
            } else {
                self.reset();
                return self.elapsed_time;
            };
        }

        if let Some(paused_time) = self.paused_time {
            elapsed -= if let Ok(duration) = SystemTime::now().duration_since(paused_time) {
                duration
            } else {
                self.reset();
                return self.elapsed_time;
            }
        }

        elapsed
    }

    /// Returns a slice of the lap times for the stopwatch.
    #[must_use]
    pub fn lap_times(&self) -> &[LapTime] {
        self.lap_times.as_slice()
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

///  This function takes a `Duration` as input and returns a `String` formatted as "mm:ss.ms". The `mm` represents minutes, `ss` represents seconds, and `ms` represents milliseconds.
#[must_use]
pub fn format_time(duration: Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    format!("{:02}:{:02}.{:02}", secs / 60, secs % 60, millis / 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stopwatch_start_stop() {
        let mut stopwatch = Stopwatch::new();

        assert!(!stopwatch.is_running());
        assert_eq!(stopwatch.elapsed_time(), Duration::from_secs(0));

        stopwatch.start();
        std::thread::sleep(Duration::from_secs(1));
        stopwatch.stop();

        assert!(!stopwatch.is_running());
        assert_eq!(stopwatch.elapsed_time().as_secs(), 1);
    }

    #[test]
    fn test_stopwatch_reset() {
        let mut stopwatch = Stopwatch::new();

        stopwatch.start();
        std::thread::sleep(Duration::from_secs(1));
        stopwatch.stop();

        assert!(!stopwatch.is_running());
        assert_eq!(stopwatch.elapsed_time().as_secs(), 1);

        stopwatch.reset();

        assert!(!stopwatch.is_running());
        assert_eq!(stopwatch.elapsed_time(), Duration::from_secs(0));
    }

    #[test]
    fn test_stopwatch_add_lap() {
        let mut stopwatch = Stopwatch::new();

        stopwatch.start();
        std::thread::sleep(Duration::from_secs(1));
        stopwatch.add_lap();
        std::thread::sleep(Duration::from_secs(2));
        stopwatch.add_lap();
        std::thread::sleep(Duration::from_secs(3));
        stopwatch.stop();

        let lap_times = stopwatch.lap_times();

        assert_eq!(lap_times.len(), 2);

        assert_eq!(lap_times[0].lap_number(), 1);
        assert_eq!(lap_times[0].split_time().as_secs(), 1);
        assert_eq!(lap_times[0].total_time().as_secs(), 1);

        assert_eq!(lap_times[1].lap_number(), 2);
        assert_eq!(lap_times[1].split_time().as_secs(), 2);
        assert_eq!(lap_times[1].total_time().as_secs(), 3);
    }

    #[test]
    fn test_stopwatch_format_time() {
        let duration = Duration::new(65, 120_000_000);

        assert_eq!(format_time(duration), "01:05.12");
    }

    #[test]
    fn test_elapsed_time() {
        let mut timer = Stopwatch::new();
        timer.start();
        std::thread::sleep(std::time::Duration::from_secs(2));
        timer.stop();
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(timer.elapsed_time().as_secs(), 2);

        timer.start();
        std::thread::sleep(std::time::Duration::from_secs(2));
        assert_eq!(timer.elapsed_time().as_secs(), 4);

        timer.reset();
        assert_eq!(timer.elapsed_time().as_secs(), 0);

        timer.start();
        std::thread::sleep(std::time::Duration::from_secs(1));
        timer.add_lap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        timer.stop();
        assert_eq!(timer.elapsed_time().as_secs(), 2);
    }
}
