use std::time::{Duration, Instant};

#[repr(C)]
#[derive(Debug)]
pub struct LapTime {
    lap_number: u32,
    split_time: Duration,
    total_time: Duration,
}

impl LapTime {
    pub fn new(lap_number: u32, split_time: Duration, total_time: Duration) -> Self {
        Self {
            lap_number,
            split_time,
            total_time,
        }
    }

    pub fn lap_number(&self) -> u32 {
        self.lap_number
    }

    pub fn split_time(&self) -> Duration {
        self.split_time
    }

    pub fn total_time(&self) -> Duration {
        self.total_time
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Stopwatch {
    running: bool,
    total_start_time: Instant,
    split_start_time: Instant,
    elapsed_time: Duration,
    paused_time: Option<Instant>,
    lap_times: Vec<LapTime>,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {
            running: false,
            total_start_time: Instant::now(),
            split_start_time: Instant::now(),
            elapsed_time: Duration::from_secs(0),
            paused_time: None,
            lap_times: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        if !self.running {
            if let Some(paused_time) = self.paused_time {
                self.total_start_time += Instant::now() - paused_time;
                self.split_start_time += Instant::now() - paused_time;
            } else {
                self.total_start_time = Instant::now();
                self.split_start_time = Instant::now();
            }
            self.running = true;
            self.paused_time = None;
        }
    }

    pub fn stop(&mut self) {
        if self.running {
            self.paused_time = Some(Instant::now());
            self.elapsed_time = self.total_start_time.elapsed();
            self.running = false;
        }
    }

    pub fn reset(&mut self) {
        self.running = false;
        self.paused_time = None;
        self.elapsed_time = Duration::from_secs(0);
        self.lap_times.clear();
    }

    pub fn add_lap(&mut self) {
        let now = Instant::now();
        if self.running {
            let lap_time = LapTime {
                lap_number: self.lap_times.len() as u32 + 1,
                split_time: now.duration_since(self.split_start_time),
                total_time: self.total_start_time.elapsed(),
            };

            self.lap_times.push(lap_time);
            self.elapsed_time = self.total_start_time.elapsed();
        }

        self.split_start_time = now;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn elapsed_time(&self) -> Duration {
        if self.running {
            self.elapsed_time + self.total_start_time.elapsed() - self.split_start_time.elapsed()
        } else {
            self.elapsed_time
        }
    }

    pub fn lap_times(&self) -> &[LapTime] {
        self.lap_times.as_ref()
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

pub fn format_time(duration: Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    format!("{:02}:{:02}.{:03}", secs / 60, secs % 60, millis)
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
        let duration = Duration::new(65, 123_000_000);

        assert_eq!(format_time(duration), "01:05.123");
    }
}
