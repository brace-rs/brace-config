use std::collections::HashMap;

use brace_config::file;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Data {
    one: String,
    two: HashMap<String, String>,
    three: Vec<usize>,
}

#[test]
fn test_file_json() {
    let data = file::load::<Data, _>("tests/assets/example.json").unwrap();

    assert_eq!(data.one, "Hello world");
    assert_eq!(data.two.len(), 2);
    assert_eq!(data.two.get("a"), Some(&"first".to_owned()));
    assert_eq!(data.two.get("b"), Some(&"second".to_owned()));
    assert_eq!(data.three.len(), 3);
    assert_eq!(data.three[0], 1);
    assert_eq!(data.three[1], 25);
    assert_eq!(data.three[2], 150);

    file::save("tests/outputs/example.json", &data).unwrap();

    let data = file::load::<Data, _>("tests/outputs/example.json").unwrap();

    assert_eq!(data.one, "Hello world");
    assert_eq!(data.two.len(), 2);
    assert_eq!(data.two.get("a"), Some(&"first".to_owned()));
    assert_eq!(data.two.get("b"), Some(&"second".to_owned()));
    assert_eq!(data.three.len(), 3);
    assert_eq!(data.three[0], 1);
    assert_eq!(data.three[1], 25);
    assert_eq!(data.three[2], 150);
}

#[test]
fn test_file_toml() {
    let data = file::load::<Data, _>("tests/assets/example.toml").unwrap();

    assert_eq!(data.one, "Hello world");
    assert_eq!(data.two.len(), 2);
    assert_eq!(data.two.get("a"), Some(&"first".to_owned()));
    assert_eq!(data.two.get("b"), Some(&"second".to_owned()));
    assert_eq!(data.three.len(), 3);
    assert_eq!(data.three[0], 1);
    assert_eq!(data.three[1], 25);
    assert_eq!(data.three[2], 150);

    file::save("tests/outputs/example.toml", &data).unwrap();

    let data = file::load::<Data, _>("tests/outputs/example.toml").unwrap();

    assert_eq!(data.one, "Hello world");
    assert_eq!(data.two.len(), 2);
    assert_eq!(data.two.get("a"), Some(&"first".to_owned()));
    assert_eq!(data.two.get("b"), Some(&"second".to_owned()));
    assert_eq!(data.three.len(), 3);
    assert_eq!(data.three[0], 1);
    assert_eq!(data.three[1], 25);
    assert_eq!(data.three[2], 150);
}

#[test]
fn test_file_yaml() {
    let data = file::load::<Data, _>("tests/assets/example.yaml").unwrap();

    assert_eq!(data.one, "Hello world");
    assert_eq!(data.two.len(), 2);
    assert_eq!(data.two.get("a"), Some(&"first".to_owned()));
    assert_eq!(data.two.get("b"), Some(&"second".to_owned()));
    assert_eq!(data.three.len(), 3);
    assert_eq!(data.three[0], 1);
    assert_eq!(data.three[1], 25);
    assert_eq!(data.three[2], 150);

    file::save("tests/outputs/example.yaml", &data).unwrap();

    let data = file::load::<Data, _>("tests/outputs/example.yaml").unwrap();

    assert_eq!(data.one, "Hello world");
    assert_eq!(data.two.len(), 2);
    assert_eq!(data.two.get("a"), Some(&"first".to_owned()));
    assert_eq!(data.two.get("b"), Some(&"second".to_owned()));
    assert_eq!(data.three.len(), 3);
    assert_eq!(data.three[0], 1);
    assert_eq!(data.three[1], 25);
    assert_eq!(data.three[2], 150);
}

#[test]
fn test_file_none() {
    let res = file::load::<Data, _>("tests/assets/example");

    assert!(res.is_err());

    let data = Data {
        one: String::new(),
        two: HashMap::new(),
        three: Vec::new(),
    };

    let res = file::save("tests/outputs/example", &data);

    assert!(res.is_err());
}

#[test]
fn test_file_invalid() {
    let res = file::load::<Data, _>("tests/assets/example.txt");

    assert!(res.is_err());

    let data = Data {
        one: String::new(),
        two: HashMap::new(),
        three: Vec::new(),
    };

    let res = file::save("tests/outputs/example.txt", &data);

    assert!(res.is_err());
}
