use std::ops::Deref;

pub struct NonEmptyString(String);

impl Deref for NonEmptyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
