use std::sync::Mutex;

use chrono::Utc;
use once_cell::sync::Lazy;
use rand::{SeedableRng, rngs::StdRng};
pub static RNG: Lazy<Mutex<StdRng>> = Lazy::new(|| {
    let seed_str = std::env::var("TEST_SEED")
        .unwrap_or_else(|_| Utc::now().timestamp_nanos_opt().unwrap_or(0).to_string());

    let seed = seed_str
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("Invalid TEST_SEED value: {}", seed_str));

    println!("The global seed: {}", seed);
    Mutex::new(StdRng::seed_from_u64(seed))
});
