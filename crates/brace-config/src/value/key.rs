use std::collections::VecDeque;
use std::iter::Iterator;

#[derive(Clone, Debug, PartialEq)]
pub struct Key(VecDeque<String>);

impl Key {
    pub fn peek(&self) -> Option<&str> {
        self.0.get(0).map(AsRef::as_ref)
    }
}

impl Iterator for Key {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl From<usize> for Key {
    fn from(from: usize) -> Self {
        Key(VecDeque::from(vec![from.to_string()]))
    }
}

impl From<&str> for Key {
    fn from(from: &str) -> Self {
        Key(from.split('.').map(ToOwned::to_owned).collect())
    }
}

impl From<String> for Key {
    fn from(from: String) -> Self {
        Key(from.split('.').map(ToOwned::to_owned).collect())
    }
}
