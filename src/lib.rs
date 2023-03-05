//! # app_properties
//! A Rust library for reading application properties from a file.
//! The properties file, named 'app.properties', must be placed in
//! the same folder as the binary that uses it and follows the
//! [YAML](https://yaml.org/) pattern.
//!
//! ## Properties file example:
//! ```yaml
//! server: localhost
//! port: 8080
//! ```
//!
//! ## Using the lib:
//! ```rust
//! let properties: AppProperties = AppProperties::new();
//! let server = properties.get("server");
//! let port = properties.get("port");
//! ```

use std::{collections::HashMap, fs};

#[derive(Clone, Debug, PartialEq)]
pub struct AppProperties {
    props_map: HashMap<String, String>,
}

impl AppProperties {
    /// Create a new AppProperties.
    pub fn new() -> Self {
        let contents =
            fs::read_to_string("app.properties").expect("Couldn't find 'app.properties' file.");

        let error_msg =
            "Invalid properties file, must be a valid yaml file. Check if there is a space between ':' and property value.";

        Self {
            props_map: serde_yaml::from_str(contents.as_str()).expect(error_msg),
        }
    }

    /// Get a value for a key.
    pub fn get(&self, key: &str) -> String {
        let value = match self.props_map.get(key) {
            Some(data) => data,
            None => return "".to_string(),
        };
        value.to_string()
    }
}

#[cfg(test)]
mod test;
