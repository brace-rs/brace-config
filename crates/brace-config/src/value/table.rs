use std::collections::hash_map::{HashMap, IntoIter, Iter, IterMut};
use std::fmt;

use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, Serializer};

use super::{de::ValueDeserializer, ser::ValueSerializer, Error, Key, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Table(HashMap<String, Value>);

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<'de, K, V>(&'de self, key: K) -> Result<V, Error>
    where
        K: Into<Key>,
        V: 'de + Deserialize<'de>,
    {
        let mut key = key.into();

        match key.next() {
            Some(head) => match self.0.get(&head) {
                Some(val) => match key.peek() {
                    Some(_) => val.get(key),
                    None => Ok(V::deserialize(ValueDeserializer::new(val))?),
                },
                None => Err(Error::custom(format!("missing value for key '{}'", head))),
            },
            None => Err(Error::custom("empty key")),
        }
    }

    pub fn set<K, V>(&mut self, key: K, val: V) -> Result<&mut Table, Error>
    where
        K: Into<Key>,
        V: Serialize,
    {
        let mut key = key.into();

        match key.next() {
            Some(head) => {
                let item = self.0.entry(head).or_insert_with(Value::entry);

                match key.peek() {
                    Some(_) => {
                        item.set(key, val)?;

                        Ok(self)
                    }
                    None => {
                        *item = val.serialize(ValueSerializer)?;

                        Ok(self)
                    }
                }
            }
            None => Err(Error::custom("empty key")),
        }
    }
}

impl Default for Table {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl Serialize for Table {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;

        for (key, value) in &self.0 {
            map.serialize_entry(&key, &value)?;
        }

        map.end()
    }
}

impl<'de> Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableVisitor;

        impl<'de> Visitor<'de> for TableVisitor {
            type Value = Table;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid table")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut map = HashMap::new();

                while let Some(key) = visitor.next_key()? {
                    map.insert(key, visitor.next_value()?);
                }

                Ok(Table(map))
            }
        }

        deserializer.deserialize_any(TableVisitor)
    }
}

impl IntoIterator for Table {
    type Item = (String, Value);
    type IntoIter = IntoIter<String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Table {
    type Item = (&'a String, &'a Value);
    type IntoIter = Iter<'a, String, Value>;

    fn into_iter(self) -> Iter<'a, String, Value> {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Table {
    type Item = (&'a String, &'a mut Value);
    type IntoIter = IterMut<'a, String, Value>;

    fn into_iter(self) -> IterMut<'a, String, Value> {
        self.0.iter_mut()
    }
}

impl From<HashMap<String, Value>> for Table {
    fn from(map: HashMap<String, Value>) -> Self {
        Self(map)
    }
}

#[cfg(test)]
mod tests {
    use super::Table;

    #[test]
    fn test_table() {
        let mut table = Table::new();

        assert!(table.set("username", "joe.bloggs").is_ok());
        assert!(table.set("password", "hunter2").is_ok());
        assert!(table.set("age", "42").is_ok());

        assert_eq!(
            table.get::<_, String>("username"),
            Ok(String::from("joe.bloggs"))
        );
        assert_eq!(
            table.get::<_, String>(String::from("password")),
            Ok(String::from("hunter2"))
        );
        assert_eq!(table.get::<_, String>("age"), Ok(String::from("42")));
        assert_eq!(table.get::<_, i32>("age"), Ok(42));
    }
}
