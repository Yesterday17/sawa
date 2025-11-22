use std::ops::Deref;

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn new(s: String) -> Result<Self, EmptyStringError> {
        if s.is_empty() {
            Err(EmptyStringError::InputShouldBeNonEmpty)
        } else {
            Ok(Self(s))
        }
    }

    pub unsafe fn new_unchecked(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Error, Debug)]
pub enum EmptyStringError {
    #[error("String cannot be empty")]
    InvalidDbString,

    #[error("Input should be non-empty")]
    InputShouldBeNonEmpty,
}

impl TryFrom<String> for NonEmptyString {
    type Error = EmptyStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(EmptyStringError::InvalidDbString)
        } else {
            Ok(NonEmptyString(value))
        }
    }
}

impl TryFrom<&str> for NonEmptyString {
    type Error = EmptyStringError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(EmptyStringError::InvalidDbString)
        } else {
            Ok(NonEmptyString(value.to_string()))
        }
    }
}

impl From<NonEmptyString> for String {
    fn from(value: NonEmptyString) -> Self {
        value.0
    }
}

impl Deref for NonEmptyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
