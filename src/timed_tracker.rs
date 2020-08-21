use std::time::{Duration, Instant};

pub struct TimedTracker {
    interval: Duration,
    last: Instant,
}

impl TimedTracker {
    pub fn new(interval: Duration) -> TimedTracker {
        TimedTracker {
            interval,
            last: Instant::now(),
        }
    }

    pub fn can(&self) -> bool {
        return self.last.elapsed() > self.interval;
    }

    pub fn track(&mut self) {
        self.last = Instant::now();
    }
}
