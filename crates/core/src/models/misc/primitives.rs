use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub unsafe fn new_unchecked(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for NonEmptyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
