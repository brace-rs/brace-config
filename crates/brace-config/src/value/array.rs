use std::fmt;
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};

use super::{de::ValueDeserializer, ser::ValueSerializer, Error, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Array(Vec<Value>);

impl Array {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<'de, K, V>(&'de self, key: K) -> Result<V, Error>
    where
        K: Into<usize>,
        V: 'de + Deserialize<'de>,
    {
        let key = key.into();

        match self.0.get(key) {
            Some(value) => V::deserialize(ValueDeserializer::new(value)).map_err(Error::custom),
            None => Err(Error::custom(format!("missing value for '{}'", key))),
        }
    }

    pub fn set<K, V>(&mut self, key: K, value: V) -> Result<&mut Self, Error>
    where
        K: Into<usize>,
        V: Serialize,
    {
        self.0.insert(
            key.into(),
            value.serialize(ValueSerializer).map_err(Error::custom)?,
        );

        Ok(self)
    }
}

impl Default for Array {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl From<Vec<Value>> for Array {
    fn from(vec: Vec<Value>) -> Self {
        Self(vec)
    }
}

impl Serialize for Array {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;

        for element in &self.0 {
            seq.serialize_element(&element)?;
        }

        seq.end()
    }
}

impl<'de> Deserialize<'de> for Array {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        pub struct ArrayVisitor;

        impl<'de> Visitor<'de> for ArrayVisitor {
            type Value = Array;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid array")
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

                Ok(Array(vec))
            }
        }

        deserializer.deserialize_any(ArrayVisitor)
    }
}

impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = &'a Value;
    type IntoIter = Iter<'a, Value>;

    fn into_iter(self) -> Iter<'a, Value> {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Array {
    type Item = &'a mut Value;
    type IntoIter = IterMut<'a, Value>;

    fn into_iter(self) -> IterMut<'a, Value> {
        self.0.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::Array;

    #[test]
    fn test_array() {
        let mut array = Array::new();

        assert!(array.set(0 as usize, "joe.bloggs").is_ok());
        assert!(array.set(1 as usize, "hunter2").is_ok());
        assert!(array.set(2 as usize, "42").is_ok());

        assert_eq!(
            array.get::<_, String>(0 as usize),
            Ok(String::from("joe.bloggs"))
        );
        assert_eq!(
            array.get::<_, String>(1 as usize),
            Ok(String::from("hunter2"))
        );
        assert_eq!(array.get::<_, String>(2 as usize), Ok(String::from("42")));
        assert_eq!(array.get::<_, i32>(2 as usize), Ok(42));
    }
}
