use super::RNG;
use base64::{Engine as _, engine::general_purpose};
use rand::{Rng, RngCore, distr::Alphanumeric};

pub fn rand_string(len: usize) -> String {
    let rng = RNG.lock().unwrap();
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn rand_b64(len: usize) -> String {
    let mut bytes = vec![0u8; len];
    RNG.lock().unwrap().fill_bytes(&mut bytes);
    general_purpose::URL_SAFE_NO_PAD.encode(&bytes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unique_random_strings() {
        let mut v = Vec::with_capacity(10);
        for _ in 0..10 {
            let new = rand_string(20);
            assert!(!v.contains(&new), "Duplicate found: {}", new);
            v.push(new);
        }
    }

    #[test]
    fn test_unique_random_b64() {
        let mut v: Vec<String> = Vec::with_capacity(10);
        for _ in 0..10 {
            let new = rand_b64(20);
            assert!(!v.contains(&new), "Duplicate found: {}", new);
            v.push(new);
        }
    }
}
