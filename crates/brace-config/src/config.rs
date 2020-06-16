use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::file::{load, save};
use crate::value::{Error, Key, Table};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Config(Table);

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<'de, K, V>(&'de self, key: K) -> Result<V, Error>
    where
        K: Into<Key>,
        V: 'de + Deserialize<'de>,
    {
        self.0.get(key)
    }

    pub fn set<K, V>(&mut self, key: K, value: V) -> Result<&mut Config, Error>
    where
        K: Into<Key>,
        V: Serialize,
    {
        self.0.set(key, value)?;

        Ok(self)
    }

    pub fn load<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        load(path.as_ref()).map_err(Error::custom)
    }

    pub fn save<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        save(path.as_ref(), &self).map_err(Error::custom)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self(Table::new())
    }
}

impl From<Table> for Config {
    fn from(table: Table) -> Self {
        Self(table)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::net::Ipv4Addr;

    use serde::{Deserialize, Serialize};

    use super::Config;

    #[test]
    fn test_boolean() {
        let mut cfg = Config::new();

        assert!(cfg.set("true", true).is_ok());
        assert!(cfg.set("false", false).is_ok());

        assert_eq!(cfg.get::<_, bool>("true"), Ok(true));
        assert_eq!(cfg.get::<_, bool>("false"), Ok(false));
    }

    #[test]
    fn test_integer_signed() {
        let mut cfg = Config::new();

        assert!(cfg.set("i8", 8 as i8).is_ok());
        assert!(cfg.set("i16", 16 as i16).is_ok());
        assert!(cfg.set("i32", 32 as i32).is_ok());
        assert!(cfg.set("i64", 64 as i64).is_ok());
        assert!(cfg.set("i128", 128 as i128).is_ok());

        assert_eq!(cfg.get::<_, i8>("i8"), Ok(8));
        assert_eq!(cfg.get::<_, i16>("i8"), Ok(8));
        assert_eq!(cfg.get::<_, i32>("i8"), Ok(8));
        assert_eq!(cfg.get::<_, i64>("i8"), Ok(8));
        assert_eq!(cfg.get::<_, i128>("i8"), Ok(8));
        assert_eq!(cfg.get::<_, String>("i8"), Ok(String::from("8")));
    }

    #[test]
    fn test_integer_unsigned() {
        let mut cfg = Config::new();

        assert!(cfg.set("u8", 8 as u8).is_ok());
        assert!(cfg.set("u16", 16 as u16).is_ok());
        assert!(cfg.set("u32", 32 as u32).is_ok());
        assert!(cfg.set("u64", 64 as u64).is_ok());
        assert!(cfg.set("u128", 128 as u128).is_ok());

        assert_eq!(cfg.get::<_, u8>("u8"), Ok(8));
        assert_eq!(cfg.get::<_, u16>("u8"), Ok(8));
        assert_eq!(cfg.get::<_, u32>("u8"), Ok(8));
        assert_eq!(cfg.get::<_, u64>("u8"), Ok(8));
        assert_eq!(cfg.get::<_, u128>("u8"), Ok(8));
        assert_eq!(cfg.get::<_, String>("u8"), Ok(String::from("8")));
    }

    #[test]
    fn test_float() {
        let mut cfg = Config::new();

        assert!(cfg.set::<_, f32>("f32", 32.0).is_ok());
        assert!(cfg.set::<_, f64>("f64", 64.0).is_ok());

        assert_eq!(cfg.get::<_, f32>("f32"), Ok(32.0 as f32));
        assert_eq!(cfg.get::<_, f64>("f64"), Ok(64.0 as f64));
    }

    #[test]
    fn test_text() {
        let mut cfg = Config::new();

        assert!(cfg.set("char", 'c').is_ok());
        assert!(cfg.set("str", "str").is_ok());
        assert!(cfg.set("string", String::from("string")).is_ok());

        assert_eq!(cfg.get::<_, char>("char"), Ok('c'));
        assert_eq!(cfg.get::<_, String>("str"), Ok(String::from("str")));
        assert_eq!(cfg.get::<_, String>("string"), Ok(String::from("string")));
    }

    #[test]
    fn test_tuple() {
        let mut cfg = Config::new();

        assert!(cfg.set("tuple", ('a', "bee", 3, false)).is_ok());

        assert_eq!(
            cfg.get::<_, (String, String, String, String)>("tuple"),
            Ok((
                String::from("a"),
                String::from("bee"),
                String::from("3"),
                String::from("false"),
            ))
        );
        assert_eq!(
            cfg.get::<_, (char, String, usize, bool)>("tuple"),
            Ok(('a', String::from("bee"), 3, false))
        );
    }

    #[test]
    fn test_seq() {
        let mut cfg = Config::new();

        assert!(cfg.set("seq", vec!["hello", "world"]).is_ok());

        assert_eq!(
            cfg.get::<_, Vec<String>>("seq"),
            Ok(vec![String::from("hello"), String::from("world")])
        );
    }

    #[test]
    fn test_map() {
        let mut cfg = Config::new();
        let mut map = HashMap::<String, Vec<String>>::new();

        map.insert(
            String::from("a"),
            vec![String::from("hello"), String::from("world")],
        );
        map.insert(String::from("b"), Vec::new());

        assert!(cfg.set("map", map.clone()).is_ok());

        assert_eq!(cfg.get::<_, HashMap<String, Vec<String>>>("map"), Ok(map));
    }

    #[test]
    fn test_struct() {
        let mut cfg = Config::new();

        #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
        struct A {
            one: String,
            two: usize,
        }

        let a = A {
            one: String::from("first"),
            two: 42,
        };

        assert!(cfg.set("struct", a.clone()).is_ok());

        assert_eq!(cfg.get::<_, A>("struct"), Ok(a));
    }

    #[test]
    fn test_unit() {
        let mut cfg = Config::new();

        #[derive(Serialize, Deserialize)]
        struct Unit;

        assert!(cfg.set("unit", ()).is_err());
        assert!(cfg.set("unit_struct", Unit).is_err());
    }

    #[test]
    fn test_enum_simple() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        #[serde(rename_all = "lowercase")]
        enum Simple {
            One,
            Two,
        }

        let mut cfg = Config::new();

        assert!(cfg.set("one", Simple::One).is_ok());
        assert!(cfg.set("two", Simple::Two).is_ok());

        assert_eq!(cfg.get::<_, String>("one"), Ok(String::from("one")));
        assert_eq!(cfg.get::<_, String>("two"), Ok(String::from("two")));

        assert_eq!(cfg.get::<_, Simple>("one"), Ok(Simple::One));
        assert_eq!(cfg.get::<_, Simple>("two"), Ok(Simple::Two));
    }

    #[test]
    fn test_enum_complex() {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        enum Complex {
            A,
            B(String),
            C(String, HashMap<String, usize>, Vec<String>),
            D {
                a: String,
            },
            E {
                a: String,
                b: HashMap<String, usize>,
                c: Vec<String>,
            },
        }

        let mut cfg = Config::new();
        let mut map = HashMap::<String, usize>::new();
        let mut arr = Vec::new();

        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);

        arr.push(String::from("a"));
        arr.push(String::from("b"));

        assert!(cfg.set("a", Complex::A).is_ok());
        assert!(cfg.set("b", Complex::B(String::from("B"))).is_ok());
        assert!(cfg
            .set("c", Complex::C(String::from("C"), map.clone(), arr.clone()))
            .is_ok());
        assert!(cfg
            .set(
                "d",
                Complex::D {
                    a: String::from("A")
                }
            )
            .is_ok());
        assert!(cfg
            .set(
                "e",
                Complex::E {
                    a: String::from("a"),
                    b: map.clone(),
                    c: arr.clone(),
                }
            )
            .is_ok());

        assert_eq!(cfg.get::<_, String>("a"), Ok(String::from("A")));
        assert_eq!(cfg.get::<_, Complex>("a"), Ok(Complex::A));
        assert_eq!(
            cfg.get::<_, Complex>("b"),
            Ok(Complex::B(String::from("B")))
        );
        assert_eq!(
            cfg.get::<_, Complex>("c"),
            Ok(Complex::C(String::from("C"), map.clone(), arr.clone()))
        );
        assert_eq!(
            cfg.get::<_, Complex>("d"),
            Ok(Complex::D {
                a: String::from("A")
            })
        );
        assert_eq!(
            cfg.get::<_, Complex>("e"),
            Ok(Complex::E {
                a: String::from("a"),
                b: map,
                c: arr,
            })
        );

        assert_eq!(cfg.get::<_, String>("a"), Ok(String::from("A")));
        assert_eq!(cfg.get::<_, String>("b.B"), Ok(String::from("B")));
        assert_eq!(cfg.get::<_, String>("c.C.0"), Ok(String::from("C")));
        assert_eq!(cfg.get::<_, String>("c.C.1.b"), Ok(String::from("2")));
        assert_eq!(cfg.get::<_, String>("c.C.2.0"), Ok(String::from("a")));
        assert_eq!(cfg.get::<_, String>("d.D.a"), Ok(String::from("A")));
        assert_eq!(cfg.get::<_, String>("e.E.c.1"), Ok(String::from("b")));
        assert_eq!(cfg.get::<_, String>("e.E.b.a"), Ok(String::from("1")));
    }

    #[test]
    fn test_ipv4() {
        let mut cfg = Config::new();

        assert!(cfg.set("ipv4", "127.0.0.1").is_ok());

        assert_eq!(
            cfg.get::<_, String>("ipv4").unwrap(),
            String::from("127.0.0.1")
        );
        assert_eq!(
            cfg.get::<_, Ipv4Addr>("ipv4").unwrap(),
            Ipv4Addr::new(127, 0, 0, 1)
        );

        assert!(cfg.set("ipv4", Ipv4Addr::new(127, 0, 0, 1)).is_ok());

        assert_eq!(cfg.get::<_, String>("ipv4"), Ok(String::from("127.0.0.1")));
        assert_eq!(
            cfg.get::<_, Ipv4Addr>("ipv4"),
            Ok(Ipv4Addr::new(127, 0, 0, 1))
        );
    }

    #[test]
    fn test_nested() {
        let mut cfg = Config::new();

        assert!(cfg.set("one", "1").is_ok());
        assert!(cfg.set("two", "2").is_ok());

        assert_eq!(cfg.get::<_, String>("one"), Ok(String::from("1")));
        assert_eq!(cfg.get::<_, String>("two"), Ok(String::from("2")));

        assert!(cfg.set("one.two", "3").is_ok());
        assert!(cfg.set("two.0", "a").is_ok());
        assert!(cfg.set("two.2", "c").is_err());
        assert!(cfg.set("two.1", "b").is_ok());
        assert!(cfg.set("two.2", "c").is_ok());

        assert_eq!(cfg.get::<_, String>("one.two"), Ok(String::from("3")));
        assert_eq!(cfg.get::<_, String>("two.0"), Ok(String::from("a")));
        assert_eq!(cfg.get::<_, String>("two.1"), Ok(String::from("b")));
        assert_eq!(cfg.get::<_, String>("two.2"), Ok(String::from("c")));

        assert!(cfg.set("one.two.three", "6").is_ok());
        assert!(cfg.set("0.0.0.a.0", "A").is_ok());
        assert!(cfg.set("0.1.0.b.0", "B").is_ok());

        assert_eq!(cfg.get::<_, String>("one.two.three"), Ok(String::from("6")));
        assert_eq!(cfg.get::<_, String>("0.0.0.a.0"), Ok(String::from("A")));
        assert_eq!(cfg.get::<_, String>("0.1.0.b.0"), Ok(String::from("B")));

        assert!(cfg.set("0.zero.0.a.0", "A").is_ok());

        assert_eq!(cfg.get::<_, String>("0.0.0.a.0"), Ok(String::from("A")));
        assert_eq!(cfg.get::<_, String>("0.zero.0.a.0"), Ok(String::from("A")));
    }
}
