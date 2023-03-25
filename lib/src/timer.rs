//! A module that defines all the core functions and structs used to create timers

use std::time::{Duration, SystemTime};

/// A timer that can be started, stopped, reset, and queried for its elapsed and remaining time.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use clock::timer::Timer;
///
/// // Create a timer with a duration of 5 seconds
/// let mut timer = Timer::new(Duration::from_secs(5));
///
/// // Start the timer
/// timer.start();
///
/// // Wait for some time (e.g., 3 seconds)
/// std::thread::sleep(Duration::from_secs(3));
///
/// // Stop the timer
/// timer.stop();
///
/// // Check that the elapsed time is approximately 3 seconds
/// assert!(timer.elapsed().as_secs_f32() >= 2.9 && timer.elapsed().as_secs_f32() <= 3.1);
///
///  timer.start();
///
/// // Wait for the remaining time (e.g., 2 seconds)
/// std::thread::sleep(timer.remaining());
///
/// // Check that the timer is now expired
/// assert_eq!(timer.remaining(), Duration::from_secs(0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timer {
    /// An [`SystemTime`] representing the time when the timer was started or last resumed.
    start_time: SystemTime,
    /// A [`Duration`] representing the total time that the timer has been running since it was last started or resumed.
    elapsed_time: Duration,
    /// A [`Duration`] representing the total duration of the timer.
    duration: Duration,
    /// A boolean indicating whether the timer is currently running or stopped.
    is_running: bool,
}

impl Timer {
    /// Creates a new [`Timer`] with zero elapsed time and in the stopped state.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use clock::timer::Timer;
    ///
    /// // Create a timer with a duration of 5 seconds
    /// let timer = Timer::new(Duration::from_secs(5));
    ///
    /// // Check that the duration is 5 seconds
    /// assert_eq!(timer.duration(), Duration::from_secs(5));
    /// ```
    #[must_use]
    pub fn new(duration: Duration) -> Timer {
        Timer {
            start_time: SystemTime::now(),
            elapsed_time: Duration::default(),
            duration,
            is_running: false,
        }
    }

    /// Starts the [`Timer`].
    ///
    /// If the [`Timer`] is already running, this method has no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use clock::timer::Timer;
    /// use std::time::Duration;
    ///
    /// let duration = Duration::from_secs(5);
    /// let mut timer = Timer::new(duration);
    ///
    /// timer.start();
    /// ```
    pub fn start(&mut self) {
        if !self.is_running {
            self.is_running = true;
            self.start_time = SystemTime::now();
        }
    }

    /// Stops the [`Timer`].
    ///
    /// If the [`Timer`] is already stopped, this method has no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use clock::timer::Timer;
    /// use std::time::Duration;
    ///
    /// let duration = Duration::from_secs(5);
    /// let mut timer = Timer::new(duration);
    ///
    /// timer.start();
    /// std::thread::sleep(std::time::Duration::from_secs(1));
    /// timer.stop();
    /// let elapsed = timer.elapsed();
    /// assert!(elapsed > Duration::from_secs(0));
    /// assert!(elapsed < Duration::from_secs(5));
    /// ```
    pub fn stop(&mut self) {
        if self.is_running {
            self.elapsed_time += self.start_time.elapsed().unwrap_or_default();
            self.is_running = false;
        }
    }

    /// Resets the [`Timer`] to its initial state.
    ///
    /// The [`Timer`] will have zero elapsed time and will be in the stopped state.
    ///
    /// # Examples
    ///
    /// ```
    /// use clock::timer::Timer;
    /// use std::time::Duration;
    ///
    /// let duration = Duration::from_secs(5);
    /// let mut timer = Timer::new(duration);
    ///
    /// timer.start();
    /// std::thread::sleep(std::time::Duration::from_secs(1));
    /// timer.reset();
    ///
    /// assert_eq!(timer.elapsed(), Duration::from_secs(0));
    /// assert_eq!(timer.remaining(), Duration::from_secs(5));
    /// ```
    pub fn reset(&mut self) {
        self.start_time = SystemTime::now();
        self.elapsed_time = Duration::default();
        self.is_running = false;
    }

    /// Returns the elapsed time since the timer was started.
    ///
    /// If the timer is currently running, the elapsed time includes the time since the timer was started
    /// plus any time that has elapsed since the timer was last stopped. If the timer is currently stopped,
    /// the elapsed time is the total time that elapsed since the timer was last started and then stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::{Duration, Instant};
    /// use clock::timer::Timer;
    ///
    /// let duration = Duration::from_secs(5);
    /// let mut timer = Timer::new(duration);
    ///
    /// timer.start();
    /// std::thread::sleep(Duration::from_secs(2));
    ///
    /// // Check the elapsed time, which should be around 3 seconds
    /// dbg!(timer.elapsed() > Duration::from_secs(2) && timer.elapsed() < Duration::from_secs(4));
    /// assert!(timer.elapsed() > Duration::from_secs(2) && timer.elapsed() < Duration::from_secs(4));
    ///
    /// // Check the remaining time, which should be around 2 seconds
    /// assert!(timer.remaining() > Duration::from_secs(1) && timer.remaining() < Duration::from_secs(3));
    ///
    /// // Stop the timer
    /// timer.stop();
    ///
    /// // Check the elapsed time again, which should be around 3 seconds
    /// assert!(timer.elapsed() > Duration::from_secs(2) && timer.elapsed() < Duration::from_secs(4));
    ///
    /// // Reset the timer to its initial state
    /// timer.reset();
    ///
    /// // Check the elapsed time again, which should be 0 seconds
    /// assert_eq!(timer.elapsed(), Duration::from_secs(0));
    ///
    /// // Check the remaining time, which should be 5 seconds
    /// assert_eq!(timer.remaining(), Duration::from_secs(5));
    /// ```
    #[must_use]
    pub fn elapsed(&mut self) -> Duration {
        let mut elapsed = self.elapsed_time;
        if self.is_running {
            elapsed += self.start_time.elapsed().unwrap_or_default();
        }
        elapsed
    }

    /// Returns the remaining time until the timer expires.
    ///
    /// If the timer is currently running, this function calculates the remaining time by subtracting
    /// the elapsed time from the total duration. If the timer is stopped or has not yet been started,
    /// the remaining time is simply the total duration of the timer.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use clock::timer::Timer;
    ///
    /// let mut timer = Timer::new(Duration::from_secs(10));
    ///
    /// // Start the timer and wait for a moment.
    /// timer.start();
    /// std::thread::sleep(Duration::from_secs(2));
    ///
    /// // Verify that there is still some time remaining on the timer.
    /// let remaining = timer.remaining();
    /// assert!(remaining.as_secs_f32() > 7.);
    /// assert!(remaining.as_secs_f32() < 8.);
    /// ```
    #[must_use]
    pub fn remaining(&self) -> Duration {
        let mut remaining = self.duration();
        if self.is_running {
            remaining = remaining
                .checked_sub(self.start_time.elapsed().unwrap_or_default() + self.elapsed_time)
                .unwrap_or_default();
        } else if self.elapsed_time != Duration::from_secs(0) {
            remaining = remaining.checked_sub(self.elapsed_time).unwrap_or_default();
        }
        remaining.max(Duration::from_secs(0))
    }

    /// Returns the duration that was specified when the timer was created
    /// or last reset. This duration determines how long the timer will run
    /// before it expires. This function is thread-safe and can be called from
    /// multiple threads simultaneously.ted.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use clock::timer::Timer;
    ///
    /// let duration = Duration::from_secs(5);
    /// let mut timer = Timer::new(duration);
    ///
    /// assert_eq!(timer.duration(), duration);
    /// ```
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns if the timer is running
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Returns if the timer is done
    #[must_use]
    pub fn is_done(&self) -> bool {
        self.remaining().is_zero()
    }

    /// Returns if the timer has not started
    #[must_use]
    pub fn has_not_started(&self) -> bool {
        self.remaining() == self.duration()
    }
}

#[cfg(test)]
mod tests {
    use super::Timer;

    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_new_timer() {
        let duration = Duration::from_secs(10);
        let mut timer = Timer::new(duration);
        assert_eq!(timer.elapsed(), Duration::default());
        assert_eq!(timer.duration(), duration);
    }

    #[test]
    fn test_start_and_stop_timer() {
        let duration = Duration::from_secs(2);
        let mut timer = Timer::new(duration);
        timer.start();
        thread::sleep(Duration::from_secs(1));
        timer.stop();
        let elapsed = timer.elapsed();
        assert!(elapsed > Duration::from_secs(0));
        assert!(elapsed < duration);
        assert!(timer.remaining() < duration);
    }

    #[test]
    fn test_remaining_start() {
        let duration = Duration::from_secs(10);
        let mut timer = Timer::new(duration);
        timer.start();
        thread::sleep(Duration::from_secs(3));
        timer.stop();
        timer.start();
        dbg!(timer.remaining());
        assert!(timer.remaining() < Duration::from_secs(7));
    }

    #[test]
    fn test_reset_timer() {
        let duration = Duration::from_secs(2);
        let mut timer = Timer::new(duration);
        timer.start();
        thread::sleep(Duration::from_secs(1));
        timer.reset();
        assert_eq!(timer.elapsed(), Duration::default());
    }

    #[test]
    fn test_remaining() {
        let duration = Duration::from_secs(10);
        let mut timer = Timer::new(duration);

        // Before starting the timer, remaining time should equal the duration
        assert_eq!(timer.remaining(), duration);

        // After starting the timer, remaining time should be less than the duration
        timer.start();
        assert!(timer.remaining() < duration);

        // After stopping the timer, remaining time should be equal to the duration minus the elapsed time
        timer.stop();
        let elapsed_time = timer.elapsed();
        let expected_remaining = duration.checked_sub(elapsed_time).unwrap_or_default();
        assert_eq!(timer.remaining(), expected_remaining);

        // After resetting the timer, remaining time should equal the duration again
        timer.reset();
        assert_eq!(timer.remaining(), duration);
    }

    #[test]
    fn test_concurrent_use() {
        const NUM_THREADS: usize = 10;

        // Create a timer with a duration of 1 second
        let timer = Arc::new(Mutex::new(Timer::new(Duration::from_secs(1))));

        // Create a barrier to synchronize the start of all threads
        let barrier = Arc::new(Barrier::new(NUM_THREADS));

        // Create a vector to hold the threads
        let mut threads = Vec::new();

        // Spawn NUM_THREADS threads, each of which will repeatedly start and stop the timer until it expires
        for _ in 0..NUM_THREADS {
            let timer = timer.clone();
            let barrier = barrier.clone();

            let thread = thread::spawn(move || {
                // Wait for all threads to start at the same time
                barrier.wait();

                // Repeatedly start and stop the timer until it expires
                while timer.lock().unwrap().remaining() > Duration::from_millis(0) {
                    timer.lock().unwrap().start();
                    thread::sleep(Duration::from_millis(10));
                    timer.lock().unwrap().stop();
                    thread::sleep(Duration::from_millis(10));
                }
            });

            threads.push(thread);
        }

        // Wait for all threads to finish
        for thread in threads {
            thread.join().unwrap();
        }

        // Check that the timer has expired
        assert_eq!(timer.lock().unwrap().remaining(), Duration::from_secs(0));
        assert_eq!(timer.lock().unwrap().elapsed().as_secs(), 1);
    }

    #[test]
    fn test_is_done() {
        let duration = Duration::from_secs(5);
        let mut timer = Timer::new(duration);
        assert!(!timer.is_done());
        timer.start();
        std::thread::sleep(duration);
        assert!(timer.is_done());
    }

    #[test]
    fn test_has_not_started() {
        let duration = Duration::from_secs(10);
        let mut timer = Timer::new(duration);
        assert!(timer.has_not_started());
        timer.start();
        assert!(!timer.has_not_started());
        std::thread::sleep(Duration::from_secs(5));
        assert!(!timer.has_not_started());
    }
}
