use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use time::{OffsetDateTime, UtcOffset};

pub trait Clock: Send + Sync {
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
    pub fn new(time: Option<OffsetDateTime>) -> Self {
        Self {
            time: Mutex::new(time),
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

pub static GLOBAL_CLOCK: Lazy<RwLock<Arc<dyn Clock>>> =
    Lazy::new(|| RwLock::new(Arc::new(SystemClock)));

/// Set a new clock implementation globally.
/// Use this in tests to inject StubClock, etc.
pub fn set_global_clock<T: Clock + 'static>(clock: T) {
    let mut global = GLOBAL_CLOCK.write().unwrap();
    *global = Arc::new(clock);
}

/// Get the current clock globally (read-only clone)
pub fn global_clock() -> Arc<dyn Clock> {
    GLOBAL_CLOCK.read().unwrap().clone()
}

/// Shortcut helpers to call the global clock
pub fn now_utc() -> OffsetDateTime {
    global_clock().now_utc()
}

pub fn now_jst() -> OffsetDateTime {
    global_clock().now_jst()
}

#[cfg(test)]
mod tests {
    use time::{OffsetDateTime, format_description::well_known::Rfc3339};

    use crate::time::clock::{Clock, StubClock, SystemClock, set_global_clock};
    use std::time::Duration;

    use super::now_utc;

    fn parse_datetime(s: &str) -> OffsetDateTime {
        OffsetDateTime::parse(s, &Rfc3339).expect("invalid datetime format")
    }

    #[test]
    fn global_clock_should_return_system_time() {
        let now = now_utc();
        let system_time = SystemClock.now_utc();
        assert!(system_time - now < Duration::from_secs(2));
    }

    #[test]
    fn should_get_set_time_from_stub_clock() {
        let dt = parse_datetime("2024-01-01T00:00:00Z");
        let stub = StubClock::new(Some(dt));

        assert_eq!(stub.now_utc(), dt);
    }

    #[test]
    fn should_advance_time_in_stub_clock() {
        let dt = parse_datetime("2024-01-01T00:00:00Z");
        let advance_by = Duration::from_secs(5 * 3600);

        let stub = StubClock::new(Some(dt));
        stub.advance(advance_by);

        assert_eq!(stub.now_utc(), dt + advance_by);
    }

    #[test]
    fn should_fallback_to_system_clock_if_not_set() {
        let stub = StubClock::new(None);
        let system_now = OffsetDateTime::now_utc();
        let stub_now = stub.now_utc();

        // Allow small drift between system and stub fallback
        assert!((stub_now - system_now).abs() < Duration::from_secs(1));
    }

    #[test]
    fn should_convert_utc_to_jst() {
        let dt = parse_datetime("2024-01-01T00:00:00Z"); // UTC
        let expected_jst = parse_datetime("2024-01-01T09:00:00+09:00"); // JST
        let stub = StubClock::new(Some(dt));

        assert_eq!(stub.now_jst(), expected_jst);
    }

    #[test]
    fn should_use_global_stub_clock() {
        let dt = parse_datetime("2024-01-01T00:00:00Z");
        let stub = StubClock::new(Some(dt));

        set_global_clock(stub);

        assert_eq!(now_utc(), dt);
    }

    #[test]
    fn global_clock_should_switch_back_to_system_clock() {
        let dt = parse_datetime("2024-01-01T00:00:00Z");
        let stub = StubClock::new(Some(dt));

        set_global_clock(stub);
        assert_eq!(now_utc(), dt);

        set_global_clock(SystemClock);
        let system_now = OffsetDateTime::now_utc();
        let actual = now_utc();

        assert!((actual - system_now).abs() < Duration::from_secs(1));
    }
}
