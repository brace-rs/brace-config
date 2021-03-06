use std::error::Error as StdError;
use std::fmt::{self, Display};

use serde::de::value::{MapDeserializer, SeqDeserializer};
use serde::de::{
    Deserialize, DeserializeSeed, Deserializer, EnumAccess, Error as DeError, IntoDeserializer,
    Unexpected, VariantAccess, Visitor,
};
use serde::forward_to_deserialize_any;

use super::{Array, Entry, Table, Value};

pub struct ValueDeserializer<'de>(&'de Value);

impl<'de> ValueDeserializer<'de> {
    pub fn new(value: &'de Value) -> Self {
        Self(value)
    }

    pub fn deserialize_entry<V>(self, entry: &'de Entry, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(&entry.0)
    }

    pub fn deserialize_array<V>(self, array: &'de Array, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let mut deserializer = SeqDeserializer::new(array.into_iter());
        let seq = visitor.visit_seq(&mut deserializer)?;

        deserializer.end()?;

        Ok(seq)
    }

    pub fn deserialize_table<V>(self, table: &'de Table, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        let iter = table
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value));
        let mut deserializer = MapDeserializer::new(iter);
        let map = visitor.visit_map(&mut deserializer)?;

        deserializer.end()?;

        Ok(map)
    }
}

impl<'de> Deserializer<'de> for ValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Entry(entry) => self.deserialize_entry(entry, visitor),
            Value::Array(array) => self.deserialize_array(array, visitor),
            Value::Table(table) => self.deserialize_table(table, visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as bool")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as bool")),
            Value::Entry(entry) => match entry.0.parse::<bool>() {
                Ok(value) => visitor.visit_bool(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as i8")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as i8")),
            Value::Entry(entry) => match entry.0.parse::<i8>() {
                Ok(value) => visitor.visit_i8(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as i16")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as i16")),
            Value::Entry(entry) => match entry.0.parse::<i16>() {
                Ok(value) => visitor.visit_i16(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as i32")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as i32")),
            Value::Entry(entry) => match entry.0.parse::<i32>() {
                Ok(value) => visitor.visit_i32(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as i64")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as i64")),
            Value::Entry(entry) => match entry.0.parse::<i64>() {
                Ok(value) => visitor.visit_i64(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as i128")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as i128")),
            Value::Entry(entry) => match entry.0.parse::<i128>() {
                Ok(value) => visitor.visit_i128(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as u8")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as u8")),
            Value::Entry(entry) => match entry.0.parse::<u8>() {
                Ok(value) => visitor.visit_u8(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as u16")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as u16")),
            Value::Entry(entry) => match entry.0.parse::<u16>() {
                Ok(value) => visitor.visit_u16(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as u32")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as u32")),
            Value::Entry(entry) => match entry.0.parse::<u32>() {
                Ok(value) => visitor.visit_u32(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as u64")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as u64")),
            Value::Entry(entry) => match entry.0.parse::<u64>() {
                Ok(value) => visitor.visit_u64(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as u128")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as u128")),
            Value::Entry(entry) => match entry.0.parse::<u128>() {
                Ok(value) => visitor.visit_u128(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as f32")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as f32")),
            Value::Entry(entry) => match entry.0.parse::<f32>() {
                Ok(value) => visitor.visit_f32(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as f64")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as f64")),
            Value::Entry(entry) => match entry.0.parse::<f64>() {
                Ok(value) => visitor.visit_f64(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as char")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as char")),
            Value::Entry(entry) => match entry.0.parse::<char>() {
                Ok(value) => visitor.visit_char(value),
                Err(err) => Err(Error::custom(format!("{}", err))),
            },
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as str")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as str")),
            Value::Entry(entry) => visitor.visit_str(&entry.0),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Array(_) => Err(Error::custom("cannot deserialize array variant as string")),
            Value::Table(_) => Err(Error::custom("cannot deserialize table variant as string")),
            Value::Entry(entry) => visitor.visit_str(&entry.0),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (variant, value) = match self.0 {
            Value::Entry(entry) => (&entry.0, None),
            Value::Table(table) => {
                let mut iter = table.into_iter();

                let (variant, value) = match iter.next() {
                    Some(v) => v,
                    None => {
                        return Err(Error::invalid_value(
                            Unexpected::Map,
                            &"map with a single key",
                        ));
                    }
                };

                if iter.next().is_some() {
                    return Err(Error::invalid_value(
                        Unexpected::Map,
                        &"map with a single key",
                    ));
                }

                (variant, Some(value))
            }
            other => {
                return Err(Error::invalid_type(other.unexpected(), &"string or map"));
            }
        };

        visitor.visit_enum(EnumDeserializer { variant, value })
    }

    forward_to_deserialize_any! {
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}

impl Value {
    fn unexpected(&self) -> Unexpected {
        match *self {
            Value::Entry(ref s) => Unexpected::Str(&s.0),
            Value::Array(_) => Unexpected::Seq,
            Value::Table(_) => Unexpected::Map,
        }
    }
}

struct EnumDeserializer<'de> {
    variant: &'de str,
    value: Option<&'de Value>,
}

impl<'de> EnumAccess<'de> for EnumDeserializer<'de> {
    type Error = Error;
    type Variant = VariantDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        let visitor = VariantDeserializer { value: self.value };
        seed.deserialize(variant).map(|v| (v, visitor))
    }
}

struct VariantDeserializer<'de> {
    value: Option<&'de Value>,
}

impl<'de> VariantAccess<'de> for VariantDeserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        match self.value {
            Some(value) => Deserialize::deserialize(ValueDeserializer::new(value)),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(ValueDeserializer::new(value)),
            None => Err(Error::invalid_type(
                Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Array(array)) => {
                Deserializer::deserialize_any(SeqDeserializer::new(array.into_iter()), visitor)
            }
            Some(other) => Err(Error::invalid_type(other.unexpected(), &"tuple variant")),
            None => Err(Error::invalid_type(
                Unexpected::UnitVariant,
                &"tuple variant",
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Table(table)) => {
                let iter = table
                    .into_iter()
                    .map(|(key, value)| (key.to_owned(), value));

                Deserializer::deserialize_any(MapDeserializer::new(iter), visitor)
            }
            Some(other) => Err(Error::invalid_type(other.unexpected(), &"struct variant")),
            _ => Err(Error::invalid_type(
                Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl StdError for Error {}

impl DeError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}
