use std::collections::HashMap;
use std::fmt;

use serde::de::{
    Deserialize, DeserializeOwned, Deserializer, IntoDeserializer, MapAccess, SeqAccess, Visitor,
};
use serde::ser::{Serialize, Serializer};

use self::de::{Error as DeError, ValueDeserializer};
use self::ser::ValueSerializer;

pub use self::array::Array;
pub use self::entry::Entry;
pub use self::error::Error;
pub use self::key::Key;
pub use self::table::Table;

mod array;
mod entry;
mod error;
mod key;
mod table;

pub(crate) mod de;
pub(crate) mod ser;

pub fn from_value<T>(value: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    T::deserialize(ValueDeserializer::new(&value)).map_err(Error::custom)
}

pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: Serialize,
{
    value.serialize(ValueSerializer).map_err(Error::custom)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Entry(Entry),
    Array(Array),
    Table(Table),
}

impl Value {
    pub fn entry() -> Self {
        Value::Entry(Entry::new())
    }

    pub fn table() -> Self {
        Value::Table(Table::new())
    }

    pub fn array() -> Self {
        Value::Array(Array::new())
    }

    pub fn get<'de, K, V>(&'de self, key: K) -> Result<V, Error>
    where
        K: Into<Key>,
        V: 'de + Deserialize<'de>,
    {
        match self {
            Value::Entry(_) => Err(Error::custom("call `get` on entry variant")),
            Value::Array(array) => array.get(key),
            Value::Table(table) => table.get(key),
        }
    }

    pub fn set<K, V>(&mut self, key: K, value: V) -> Result<&mut Self, Error>
    where
        K: Into<Key>,
        V: Serialize,
    {
        let key = key.into();

        match key.peek() {
            Some(head) => match self {
                Value::Entry(_) => match head.parse::<usize>() {
                    Ok(_) => {
                        let mut array = Value::array();
                        array.set(key, value)?;
                        *self = array;

                        Ok(self)
                    }
                    Err(_) => {
                        let mut table = Value::table();
                        table.set(key, value)?;
                        *self = table;

                        Ok(self)
                    }
                },
                Value::Array(array) => match head.parse::<usize>() {
                    Ok(_) => {
                        array.set(key, value)?;

                        Ok(self)
                    }
                    Err(_) => {
                        let mut table = Value::table();
                        for (index, item) in array.into_iter().enumerate() {
                            table.set(index, item)?;
                        }
                        table.set(key, value)?;
                        *self = table;

                        Ok(self)
                    }
                },
                Value::Table(table) => {
                    table.set(key, value)?;

                    Ok(self)
                }
            },
            None => Err(Error::custom("empty key")),
        }
    }

    pub fn is_entry(&self) -> bool {
        match self {
            Value::Entry(_) => true,
            _ => false,
        }
    }

    pub fn as_entry(&self) -> Option<&Entry> {
        match self {
            Value::Entry(entry) => Some(entry),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Value::Array(_) => true,
            _ => false,
        }
    }

    pub fn as_array(&self) -> Option<&Array> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    pub fn is_table(&self) -> bool {
        match self {
            Value::Table(_) => true,
            _ => false,
        }
    }

    pub fn as_table(&self) -> Option<&Table> {
        match self {
            Value::Table(table) => Some(table),
            _ => None,
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Entry(entry) => entry.serialize(serializer),
            Value::Array(array) => array.serialize(serializer),
            Value::Table(table) => table.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        pub struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid value")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
                Ok(Value::from(value))
            }

            fn visit_string<E>(self, value: String) -> Result<Value, E> {
                Ok(Value::from(value))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }

                Ok(Value::from(vec))
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut map = HashMap::new();

                while let Some(key) = visitor.next_key()? {
                    map.insert(key, visitor.next_value()?);
                }

                Ok(Value::from(map))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl<'de> IntoDeserializer<'de, DeError> for &'de Value {
    type Deserializer = ValueDeserializer<'de>;

    fn into_deserializer(self) -> Self::Deserializer {
        ValueDeserializer::new(self)
    }
}

impl From<Entry> for Value {
    fn from(value: Entry) -> Self {
        Value::Entry(value)
    }
}

impl From<Array> for Value {
    fn from(value: Array) -> Self {
        Value::Array(value)
    }
}

impl From<Table> for Value {
    fn from(value: Table) -> Self {
        Value::Table(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Entry(Entry::from(value))
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Array(Array::from(value))
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(value: HashMap<String, Value>) -> Self {
        Value::Table(Table::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::{Array, Entry, Table, Value};

    #[test]
    fn test_entry() {
        assert!(Value::entry().is_entry());
        assert!(!Value::entry().is_array());
        assert!(!Value::entry().is_table());
        assert!(Value::from(Entry::new()).is_entry());

        assert_eq!(Value::entry().as_entry(), Some(&Entry::new()));
        assert_eq!(
            Value::from("hi").as_entry(),
            Some(&Entry(String::from("hi")))
        );
        assert_eq!(
            Value::from(String::from("hello")).as_entry(),
            Some(&Entry(String::from("hello")))
        );
    }

    #[test]
    fn test_array() {
        assert!(Value::array().is_array());
        assert!(!Value::array().is_entry());
        assert!(!Value::array().is_table());
        assert!(Value::from(Array::new()).is_array());

        assert_eq!(Value::array().as_array(), Some(&Array::new()));
    }

    #[test]
    fn test_table() {
        assert!(Value::table().is_table());
        assert!(!Value::table().is_entry());
        assert!(!Value::table().is_array());
        assert!(Value::from(Table::new()).is_table());

        assert_eq!(Value::table().as_table(), Some(&Table::new()));
    }
}
