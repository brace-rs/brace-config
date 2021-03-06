use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(String);

impl Error {
    pub fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for Error {}

impl From<super::ser::Error> for Error {
    fn from(from: super::ser::Error) -> Self {
        Self::custom(from)
    }
}

impl From<super::de::Error> for Error {
    fn from(from: super::de::Error) -> Self {
        Self::custom(from)
    }
}
