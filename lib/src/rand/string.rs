use super::RNG;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::{Rng, RngCore, distr::Alphanumeric};

pub fn rand_string(len: usize) -> String {
    let rng = RNG.lock().unwrap();
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

// Generate random base64 string with a desired length (including padding)
pub fn rand_b64(len: usize) -> String {
    // Ensure we generate at least `len` bytes
    let byte_len = (len * 3 + 3) / 4; // Calculate byte length to fit in base64

    let mut bytes = vec![0u8; byte_len];
    RNG.lock().unwrap().fill_bytes(&mut bytes);

    let encoded = STANDARD.encode(&bytes);

    // Now we have an encoded string which has at least `len` characters (with padding)
    // We can safely return the first `len` characters to match the requested size
    encoded[..len].to_string()
}

#[cfg(test)]
mod test {
    use base64::alphabet::BCRYPT;

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

    #[test]
    fn test_generated_correct_b64() {
        let allowed = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        for len in 0..100 {
            let salt = rand_b64(len);
            assert!(
                salt.chars().all(|c| allowed.contains(c)),
                "Invalid base64 character in salt: {}",
                salt
            );
            assert_eq!(salt.len(), len);
            println!("{}", salt)
        }
    }
}
