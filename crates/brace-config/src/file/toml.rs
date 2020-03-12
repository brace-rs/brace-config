use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use toml::{from_str, to_string_pretty, Value};

use crate::file::error::Error;

pub fn load<T, P>(path: P) -> Result<T, Error>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let string = read_to_string(path)?;
    let config = from_str::<T>(&string)?;

    Ok(config)
}

pub fn save<T, P>(path: P, value: &T) -> Result<(), Error>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let value = Value::try_from(value)?;
    let string = to_string_pretty(&value)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write_all(string.as_ref())?;

    Ok(())
}
