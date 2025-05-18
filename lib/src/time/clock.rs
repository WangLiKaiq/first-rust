use std::sync::Mutex;
use std::time::Duration;

use time::{OffsetDateTime, UtcOffset};

pub trait Clock {
    fn now(&self) -> OffsetDateTime {
        self.now_utc()
    }

    fn now_utc(&self) -> OffsetDateTime;

    fn now_jst(&self) -> OffsetDateTime {
        self.now_utc()
            .to_offset(UtcOffset::from_hms(9, 0, 0).unwrap())
    }
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now_utc(&self) -> OffsetDateTime {
        OffsetDateTime::now_utc()
    }
}

pub struct StubClock {
    time: Mutex<Option<OffsetDateTime>>,
}

impl StubClock {
    pub fn new() -> Self {
        Self {
            time: Mutex::new(None),
        }
    }

    /// Set the stub clock's current time
    pub fn set(&self, new_time: OffsetDateTime) {
        let mut time = self.time.lock().unwrap();
        *time = Some(new_time);
    }

    /// Advance the current stub time by a duration
    pub fn advance(&self, duration: Duration) {
        let mut time = self.time.lock().unwrap();
        if let Some(t) = time.as_mut() {
            *t += duration;
        } else {
            *time = Some(OffsetDateTime::now_utc() + duration);
        }
    }

    /// Internal method to get time or fallback to system clock
    fn effective_time(&self) -> OffsetDateTime {
        self.time
            .lock()
            .unwrap()
            .unwrap_or_else(OffsetDateTime::now_utc)
    }
}

impl Clock for StubClock {
    fn now_utc(&self) -> OffsetDateTime {
        self.effective_time()
    }
}
