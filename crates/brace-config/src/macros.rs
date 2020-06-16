#[macro_export]
macro_rules! config {
    () => {
        $crate::Config::new()
    };

    ($($tt:tt)+) => {
        $crate::Config::from($crate::table!($($tt)+))
    };
}

#[macro_export]
macro_rules! value {
    ([]) => {
        $crate::Value::array()
    };

    ([ $($tt:tt)+ ]) => {
        $crate::Value::from($crate::array!($($tt)+))
    };

    ({}) => {
        $crate::Value::table()
    };

    ({ $($tt:tt)+ }) => {
        $crate::Value::from($crate::table!($($tt)+))
    };

    ($other:expr) => {
        $crate::to_value(&$other).unwrap()
    };
}

#[macro_export]
macro_rules! entry {
    () => {
        $crate::Entry::new()
    };

    ($expr:expr) => {
        $crate::Entry::from($expr)
    };
}

#[macro_export]
macro_rules! array {
    (@array [$($elems:expr,)*]) => {
        std::vec![$($elems,)*]
    };

    (@array [$($elems:expr),*]) => {
        std::vec![$($elems),*]
    };

    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        $crate::array!(@array [$($elems,)* $crate::value!([$($array)*])] $($rest)*)
    };

    (@array [$($elems:expr,)*] {$($table:tt)*} $($rest:tt)*) => {
        $crate::array!(@array [$($elems,)* $crate::value!({$($table)*})] $($rest)*)
    };

    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        $crate::array!(@array [$($elems,)* $crate::value!($next),] $($rest)*)
    };

    (@array [$($elems:expr,)*] $last:expr) => {
        $crate::array!(@array [$($elems,)* $crate::value!($last)])
    };

    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        $crate::array!(@array [$($elems,)*] $($rest)*)
    };

    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        $crate::value_unexpected!($unexpected)
    };

    () => {
        $crate::Array::new()
    };

    ($($tt:tt)+) => {
        $crate::Array::from($crate::array!(@array [] $($tt)+))
    };
}

#[macro_export]
macro_rules! table {
    (@table $table:ident () () ()) => {};

    (@table $table:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        $table.insert(($($key)+).into(), $value);
        $crate::table!(@table $table () ($($rest)*) ($($rest)*));
    };

    (@table $table:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        $crate::value_unexpected!($unexpected);
    };

    (@table $table:ident [$($key:tt)+] ($value:expr)) => {
        $table.insert(($($key)+).into(), $value);
    };

    (@table $table:ident ($($key:tt)+) (= [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        $crate::table!(@table $table [$($key)+] ($crate::value!([$($array)*])) $($rest)*);
    };

    (@table $table:ident ($($key:tt)+) (= {$($next_table:tt)*} $($rest:tt)*) $copy:tt) => {
        $crate::table!(@table $table [$($key)+] ($crate::value!({$($next_table)*})) $($rest)*);
    };

    (@table $table:ident ($($key:tt)+) (= $value:expr , $($rest:tt)*) $copy:tt) => {
        $crate::table!(@table $table [$($key)+] ($crate::value!($value)) , $($rest)*);
    };

    (@table $table:ident ($($key:tt)+) (= $value:expr) $copy:tt) => {
        $crate::table!(@table $table [$($key)+] ($crate::value!($value)));
    };

    (@table $table:ident ($($key:tt)+) (=) $copy:tt) => {
        $crate::value_unexpected!("");
    };

    (@table $table:ident ($($key:tt)+) () $copy:tt) => {
        $crate::value_unexpected!("");
    };

    (@table $table:ident () (= $($rest:tt)*) ($unexpected:tt $($copy:tt)*)) => {
        $crate::value_unexpected!($unexpected);
    };

    (@table $table:ident ($($key:tt)*) (, $($rest:tt)*) ($unexpected:tt $($copy:tt)*)) => {
        $crate::value_unexpected!($unexpected);
    };

    (@table $table:ident () (($key:expr) = $($rest:tt)*) $copy:tt) => {
        $crate::table!(@table $table ($key) (= $($rest)*) (= $($rest)*));
    };

    (@table $table:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        $crate::table!(@table $table ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    () => {
        $crate::Table::new()
    };

    ($($tt:tt)+) => {
        {
            let mut table = std::collections::HashMap::new();
            $crate::table!(@table table () ($($tt)+) ($($tt)+));
            $crate::Table::from(table)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! value_unexpected {
    () => {};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_config() {
        let config1 = config! { "key" = "value" };
        let config2 = config! {};

        assert_eq!(config1.get::<_, String>("key").unwrap(), "value");
        assert!(config2.get::<_, String>("key").is_err());
    }

    #[test]
    fn test_value() {
        let entry = value!("entry");
        let array = value!(["array"]);
        let table = value!({ "table" = true });

        assert!(entry.is_entry());
        assert!(array.is_array());
        assert!(table.is_table());

        assert_eq!(entry.as_entry().unwrap().value(), "entry");
        assert_eq!(array.get::<_, String>("0").unwrap(), "array");
        assert_eq!(table.get::<_, bool>("table").unwrap(), true);
    }

    #[test]
    fn test_entry() {
        let entry1 = entry!("hello");
        let entry2 = entry!(String::from("hello"));

        assert_eq!(entry1.value(), "hello");
        assert_eq!(entry2.value(), "hello");
    }

    #[test]
    fn test_array() {
        let array1 = array![];
        let array2 = array![[]];
        let array3 = array![{}];
        let array4 = array!["a"];
        let array5 = array![[], {}, "a"];
        let array6 = array!['a', "b", ("c", "d")];

        assert_eq!(array1.len(), 0);
        assert_eq!(array2.len(), 1);
        assert_eq!(array3.len(), 1);
        assert_eq!(array4.len(), 1);
        assert_eq!(array5.len(), 3);
        assert_eq!(array6.len(), 3);
    }

    #[test]
    fn test_table() {
        let t = table! {
            "a" = "a",
            "b" = "b",
            "c" = {},
            "d" = [],
            "e" = ['f', "g", ("h", "i", true)],
            "j" = {
                "k" = "l",
                "m" = {
                    "n" = ["o", "p"],
                },
            },
            "q" = ("r", "s"),
        };

        assert_eq!(t.get::<_, String>("a").unwrap(), "a");
        assert_eq!(t.get::<_, String>("b").unwrap(), "b");
        assert_eq!(t.get::<_, String>("e.2.1").unwrap(), "i");
        assert_eq!(t.get::<_, String>("j.k").unwrap(), "l");
        assert_eq!(t.get::<_, String>("j.m.n.0").unwrap(), "o");
        assert_eq!(t.get::<_, String>("q.0").unwrap(), "r");
    }
}
