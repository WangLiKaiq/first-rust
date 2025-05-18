use lib::rand::rand_string;
use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, Clone)]
pub struct Email(SecretString);

impl Email {
    pub fn new<S: Into<String>>(input: S) -> Result<Self, anyhow::Error> {
        let s = input.into();
        if Self::is_valid(&s) {
            Ok(Self(SecretString::from(s)))
        } else {
            Err(anyhow::anyhow!("Invalid email"))
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.expose_secret()
    }

    pub fn is_valid(s: &str) -> bool {
        static EMAIL_RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
            regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap()
        });
        EMAIL_RE.is_match(s)
    }

    pub fn rand() -> Self {
        let user: String = rand_string(8);
        let domain: String = rand_string(4);

        Self(SecretString::from(format!("{}@{}.com", user, domain)))
    }
}

#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;

    use super::Email;

    #[test]
    fn should_return_wrapped_email() {
        let email = Email::new("test@gmail.com");
        assert!(email.is_ok())
    }

    #[test]
    fn should_generate_valid_email() {
        let email = Email::rand();
        let raw = email.0.expose_secret();
        assert!(Email::is_valid(raw));
        assert_eq!(raw.len(), 17);
    }
}
