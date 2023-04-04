# app_properties

A Rust library for reading application properties from a file.
The properties file, named 'app.properties', must be placed in
the same folder as the binary that uses it and follows the
[YAML](https://yaml.org/) pattern.

## Properties file example:

```yaml
server: localhost
port: 8080
```

## Using the lib:

```yaml
[dependencies]
app_properties = "0.1.2"
```

```rust
use app_properties::AppProperties;

let properties: AppProperties = AppProperties::new();
let server = properties.get("server");
let port = properties.get("port");
```
