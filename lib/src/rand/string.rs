use rand::{Rng, distr::Alphanumeric};

use super::RNG;

pub fn rand_string(len: usize) -> String {
    let rng = RNG.lock().unwrap();
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod test {
    use super::rand_string;

    #[test]
    fn test_unique_random_strings() {
        let mut v = Vec::with_capacity(10);
        for _ in 0..10 {
            let new = rand_string(20);
            assert!(!v.contains(&new), "Duplicate found: {}", new);
            v.push(new);
        }
    }
}
