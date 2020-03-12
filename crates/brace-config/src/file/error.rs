use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    ParseError(Box<dyn std::error::Error>),
    IoError(std::io::Error),
    InvalidFileType(Option<String>, PathBuf),
}

impl Error {
    pub fn invalid_file_type<P>(extension: Option<String>, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::InvalidFileType(extension, path.as_ref().into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseError(err) => write!(f, "{}", err),
            Self::IoError(err) => write!(f, "{}", err),
            Self::InvalidFileType(ext, path) => match ext {
                Some(ext) => write!(
                    f,
                    "Invalid file type '{}' for path '{:?}'",
                    ext,
                    path.display()
                ),
                None => write!(f, "Invalid file type for path '{:?}'", path.display()),
            },
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::ParseError(Box::new(error))
    }
}

#[cfg(feature = "toml")]
impl From<toml::ser::Error> for Error {
    fn from(error: toml::ser::Error) -> Self {
        Self::ParseError(Box::new(error))
    }
}

#[cfg(feature = "toml")]
impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Self::ParseError(Box::new(error))
    }
}

#[cfg(feature = "yaml")]
impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Self::ParseError(Box::new(error))
    }
}
