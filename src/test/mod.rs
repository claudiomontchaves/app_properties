use super::*;

#[test]
fn read_valid_properties_should_return_values() {
    let properties: AppProperties = AppProperties::new();
    let server = properties.get("server");
    let port = properties.get("port");
    println!("- server: {}", server);
    println!("- port: {}", port);
    assert_eq!(server, "localhost");
    assert_eq!(port, "8080");
}

#[test]
fn read_invalid_properties_should_return_empty_string() {
    let properties: AppProperties = AppProperties::new();
    let server = properties.get("prop1");
    assert_eq!(server, "");
}
