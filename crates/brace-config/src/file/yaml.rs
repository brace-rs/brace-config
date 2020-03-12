use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;

use serde::ser::Serialize;
use serde_yaml::{from_str, to_string};

use super::Error;
use crate::Config;

pub fn load<P>(path: P) -> Result<Config, Error>
where
    P: AsRef<Path>,
{
    let string = read_to_string(path)?;
    let config = from_str::<Config>(&string)?;

    Ok(config)
}

pub fn save<T, P>(path: P, value: &T) -> Result<(), Error>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let string = to_string(&value)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write_all(string.as_ref())?;

    Ok(())
}
