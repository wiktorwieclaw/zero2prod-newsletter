pub struct Password(String);

#[derive(thiserror::Error, Debug)]
pub enum PasswordParsingError {
    #[error("Password has to have at least 12 characters.")]
    TooShort,
    #[error("Password has to be shorter than 128 characters.")]
    TooLong,
}

impl Password {
    pub fn parse(inner: String) -> Result<Password, PasswordParsingError> {
        if inner.len() < 12 {
            Err(PasswordParsingError::TooShort)
        } else if inner.len() >= 128 {
            Err(PasswordParsingError::TooLong)
        } else {
            Ok(Password(inner))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl secrecy::Zeroize for Password {
    fn zeroize(&mut self) {
        self.0.zeroize()
    }
}
