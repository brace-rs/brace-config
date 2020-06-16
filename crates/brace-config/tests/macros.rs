use brace_config::{array, config, entry, table, value};

#[test]
fn test_config() {
    config!();
}

#[test]
fn test_value() {
    value!("test");
}

#[test]
fn test_entry() {
    entry!();
}

#[test]
fn test_array() {
    array!();
}

#[test]
fn test_table() {
    table!();
}
