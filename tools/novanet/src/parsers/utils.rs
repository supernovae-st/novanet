//! Shared utilities for YAML parsing.

use serde::de::DeserializeOwned;
use std::path::Path;

/// Load and parse a YAML file with proper error context.
///
/// Combines `std::fs::read_to_string` + `serde_yaml::from_str` with
/// consistent error handling via `NovaNetError::Schema`.
///
/// # Example
///
/// ```ignore
/// use crate::parsers::utils::load_yaml;
/// let doc: MyDocument = load_yaml(&path)?;
/// ```
pub fn load_yaml<T: DeserializeOwned>(path: &Path) -> crate::Result<T> {
    let content = std::fs::read_to_string(path)?;
    serde_yaml::from_str(&content).map_err(|e| crate::NovaNetError::Schema {
        path: path.display().to_string(),
        source: e,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn load_yaml_success() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "name: test\nvalue: 42").unwrap();

        #[derive(serde::Deserialize, Debug, PartialEq)]
        struct TestDoc {
            name: String,
            value: i32,
        }

        let doc: TestDoc = load_yaml(file.path()).unwrap();
        assert_eq!(doc.name, "test");
        assert_eq!(doc.value, 42);
    }

    #[test]
    fn load_yaml_file_not_found() {
        #[derive(serde::Deserialize, Debug)]
        struct TestDoc {
            _name: String,
        }

        let result: crate::Result<TestDoc> = load_yaml(Path::new("/nonexistent/file.yaml"));
        assert!(result.is_err());
    }

    #[test]
    fn load_yaml_invalid_yaml() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "invalid: [unclosed").unwrap();

        #[derive(serde::Deserialize, Debug)]
        struct TestDoc {
            _invalid: String,
        }

        let result: crate::Result<TestDoc> = load_yaml(file.path());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, crate::NovaNetError::Schema { .. }));
    }
}
