use std::time::Duration;

pub struct Hertz {
    period: Duration,
}

impl Hertz {
    pub fn new(freq: u64) -> Self {
        Self { period: Duration::from_millis(1/freq * 1000)}
    }

    pub fn period(&self) -> Duration {
        self.period
    }

    pub fn set_frequency(&mut self, freq: u64) {
        self.period = Duration::from_millis(1/freq * 1000);
    }
}

pub struct Logger {
}
