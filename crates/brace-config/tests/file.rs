use std::collections::HashMap;

use brace_config::{file, Config};

#[test]
fn test_file_json() {
    let cfg = file::load("tests/assets/example.json").unwrap();

    assert_eq!(cfg.get("one"), Ok(String::from("Hello world")));
    assert_eq!(
        cfg.get("two"),
        Ok({
            let mut map = HashMap::new();
            map.insert(String::from("a"), String::from("first"));
            map.insert(String::from("b"), String::from("second"));
            map
        })
    );
    assert_eq!(cfg.get("three"), Ok(vec![1, 25, 150]));

    file::save("tests/outputs/example.json", &cfg).unwrap();

    let cfg = file::load("tests/outputs/example.json").unwrap();

    assert_eq!(cfg.get("one"), Ok(String::from("Hello world")));
    assert_eq!(
        cfg.get("two"),
        Ok({
            let mut map = HashMap::new();
            map.insert(String::from("a"), String::from("first"));
            map.insert(String::from("b"), String::from("second"));
            map
        })
    );
    assert_eq!(cfg.get("three"), Ok(vec![1, 25, 150]));
}

#[test]
fn test_file_toml() {
    let cfg = file::load("tests/assets/example.toml").unwrap();

    assert_eq!(cfg.get("one"), Ok(String::from("Hello world")));
    assert_eq!(
        cfg.get("two"),
        Ok({
            let mut map = HashMap::new();
            map.insert(String::from("a"), String::from("first"));
            map.insert(String::from("b"), String::from("second"));
            map
        })
    );
    assert_eq!(cfg.get("three"), Ok(vec![1, 25, 150]));

    file::save("tests/outputs/example.toml", &cfg).unwrap();

    let cfg = file::load("tests/outputs/example.toml").unwrap();

    assert_eq!(cfg.get("one"), Ok(String::from("Hello world")));
    assert_eq!(
        cfg.get("two"),
        Ok({
            let mut map = HashMap::new();
            map.insert(String::from("a"), String::from("first"));
            map.insert(String::from("b"), String::from("second"));
            map
        })
    );
    assert_eq!(cfg.get("three"), Ok(vec![1, 25, 150]));
}

#[test]
fn test_file_yaml() {
    let cfg = file::load("tests/assets/example.yaml").unwrap();

    assert_eq!(cfg.get("one"), Ok(String::from("Hello world")));
    assert_eq!(
        cfg.get("two"),
        Ok({
            let mut map = HashMap::new();
            map.insert(String::from("a"), String::from("first"));
            map.insert(String::from("b"), String::from("second"));
            map
        })
    );
    assert_eq!(cfg.get("three"), Ok(vec![1, 25, 150]));

    file::save("tests/outputs/example.yaml", &cfg).unwrap();

    let cfg = file::load("tests/outputs/example.yaml").unwrap();

    assert_eq!(cfg.get("one"), Ok(String::from("Hello world")));
    assert_eq!(
        cfg.get("two"),
        Ok({
            let mut map = HashMap::new();
            map.insert(String::from("a"), String::from("first"));
            map.insert(String::from("b"), String::from("second"));
            map
        })
    );
    assert_eq!(cfg.get("three"), Ok(vec![1, 25, 150]));
}

#[test]
fn test_file_none() {
    let res = file::load("tests/assets/example");

    assert!(res.is_err());

    let cfg = Config::new();
    let res = file::save("tests/outputs/example", &cfg);

    assert!(res.is_err());
}

#[test]
fn test_file_invalid() {
    let res = file::load("tests/assets/example.txt");

    assert!(res.is_err());

    let cfg = Config::new();
    let res = file::save("tests/outputs/example.txt", &cfg);

    assert!(res.is_err());
}
