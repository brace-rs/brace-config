use std::path::Path;

use self::error::Error;
use crate::Config;

pub mod error;

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "toml")]
pub mod toml;

#[cfg(feature = "yaml")]
pub mod yaml;

pub fn load<P>(path: P) -> Result<Config, Error>
where
    P: AsRef<Path>,
{
    match path.as_ref().extension() {
        Some(ext) => match ext.to_str() {
            #[cfg(feature = "json")]
            Some("json") => self::json::load(path),
            #[cfg(feature = "toml")]
            Some("toml") => self::toml::load(path),
            #[cfg(feature = "yaml")]
            Some("yaml") => self::yaml::load(path),
            #[cfg(feature = "yaml")]
            Some("yml") => self::yaml::load(path),
            Some(ext) => Err(Error::invalid_file_type(
                Some(ext.to_string()),
                path.as_ref(),
            )),
            None => Err(Error::invalid_file_type(None, path.as_ref())),
        },
        None => Err(Error::invalid_file_type(None, path.as_ref())),
    }
}

pub fn save<P>(path: P, config: &Config) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    match path.as_ref().extension() {
        Some(ext) => match ext.to_str() {
            #[cfg(feature = "json")]
            Some("json") => self::json::save(path, config),
            #[cfg(feature = "toml")]
            Some("toml") => self::toml::save(path, config),
            #[cfg(feature = "yaml")]
            Some("yaml") => self::yaml::save(path, config),
            #[cfg(feature = "yaml")]
            Some("yml") => self::yaml::save(path, config),
            Some(ext) => Err(Error::invalid_file_type(
                Some(ext.to_string()),
                path.as_ref(),
            )),
            None => Err(Error::invalid_file_type(None, path.as_ref())),
        },
        None => Err(Error::invalid_file_type(None, path.as_ref())),
    }
}
