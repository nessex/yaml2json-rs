use crate::Style::{COMPACT, PRETTY};
use core::fmt::Debug;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Yaml2JsonError {
    #[error(transparent)]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

/// `Style` defines JSON output formats for `Yaml2Json`.
pub enum Style {
    /// `Style::COMPACT` outputs JSON on a single line.
    /// e.g. for the following YAML:
    /// ```yaml
    /// ---
    /// hello: world
    /// spec:
    ///   items:
    ///     - a
    ///     - b
    /// ```
    ///
    /// the JSON output will be:
    /// ```json
    /// {"hello":"world","spec":{"items":["a","b"]}}
    /// ```
    COMPACT,
    /// `Style::PRETTY` outputs JSON on multiple lines, with automatic indentation.
    /// e.g. for the following YAML:
    /// ```yaml
    /// ---
    /// hello: world
    /// spec:
    ///   items:
    ///     - a
    ///     - b
    /// ```
    ///
    /// the JSON output will be:
    /// ```json
    /// {
    ///   "hello": "world",
    ///   "spec": {
    ///     "items": [
    ///       "a",
    ///       "b"
    ///     ]
    ///   }
    /// }
    /// ```
    PRETTY,
}

/// Yaml2Json can convert individual YAML documents into JSON. Each instance can be configured to
/// have different styles of output.
///
/// The JSON output can be returned as a string:
/// ```
/// use yaml2json_rs::{Yaml2Json, Style};
///
/// let y2j = Yaml2Json::new(Style::COMPACT);
/// let input = "hello: world";
/// let output = y2j.document_to_string(input).unwrap();
///
/// assert_eq!(output, r#"{"hello":"world"}"#);
/// ```
///
/// Or, the JSON output can be sent to a writer:
/// ```
/// use yaml2json_rs::{Yaml2Json, Style};
/// use std::io;
///
/// let y2j = Yaml2Json::new(Style::COMPACT);
/// let input = "hello: world";
/// let mut stdout = io::stdout();
///
/// y2j.document_to_writer(input, &mut stdout);
///
/// // {"hello":"world"}
/// ```
pub struct Yaml2Json {
    style: Style,
}

impl Yaml2Json {
    /// `new()` creates a new `Yaml2Json`. It expects you to provide an output `Style`.
    /// ```
    /// use yaml2json_rs::{Yaml2Json, Style};
    ///
    /// let y2j_pretty = Yaml2Json::new(Style::PRETTY);
    /// let y2j_compact = Yaml2Json::new(Style::COMPACT);
    /// ```
    pub fn new(style: Style) -> Self {
        Self { style }
    }

    /// `document_to_string()` takes a YAML document &str and converts it to a JSON String.
    /// ```
    /// use yaml2json_rs::{Yaml2Json, Style};
    ///
    /// let y2j = Yaml2Json::new(Style::COMPACT);
    /// let input = "hello: world";
    /// let output = y2j.document_to_string(input).unwrap();
    ///
    /// assert_eq!(output, r#"{"hello":"world"}"#);
    /// ```
    pub fn document_to_string(&self, document: &str) -> Result<String, Yaml2JsonError> {
        let s: serde_json::Value = serde_yaml::from_str(document)?;

        let res = match self.style {
            COMPACT => serde_json::to_string(&s),
            PRETTY => serde_json::to_string_pretty(&s),
        };

        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into()),
        }
    }

    /// `document_to_writer()` takes a YAML document string, converts it to JSON and sends the output
    /// to the provided writer.
    ///
    /// ```
    /// use yaml2json_rs::{Yaml2Json, Style};
    /// use std::io;
    ///
    /// let y2j = Yaml2Json::new(Style::COMPACT);
    /// let input = "hello: world";
    /// let mut stdout = io::stdout();
    ///
    /// y2j.document_to_writer(input, &mut stdout);
    ///
    /// // {"hello":"world"}
    /// ```
    pub fn document_to_writer<W: io::Write>(
        &self,
        document: &str,
        w: &mut W,
    ) -> Result<(), Yaml2JsonError> {
        let s: serde_json::Value = serde_yaml::from_str(document)?;

        let res = match self.style {
            PRETTY => serde_json::to_writer_pretty(w, &s),
            COMPACT => serde_json::to_writer(w, &s),
        };

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Style, Yaml2Json};
    use std::io::Cursor;

    #[test]
    fn document_to_string_compact() {
        let yaml2json = Yaml2Json::new(Style::COMPACT);
        let input = r#"
---
abc: def
"#;
        let expected = r#"{"abc":"def"}"#;
        let res = yaml2json.document_to_string(input).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn document_to_string_pretty() {
        let yaml2json = Yaml2Json::new(Style::PRETTY);
        let input = r#"
---
abc: def
"#;
        let expected = r#"{
  "abc": "def"
}"#;
        let res = yaml2json.document_to_string(input).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn document_to_writer_compact() {
        let yaml2json = Yaml2Json::new(Style::COMPACT);
        let input = r#"
---
abc: def
"#;
        let expected = r#"{"abc":"def"}"#;

        let mut buf = Cursor::new(Vec::<u8>::new());
        yaml2json.document_to_writer(input, buf.get_mut()).unwrap();

        let res = String::from_utf8(buf.into_inner()).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn document_to_writer_pretty() {
        let yaml2json = Yaml2Json::new(Style::PRETTY);
        let input = r#"
---
abc: def
"#;
        let expected = r#"{
  "abc": "def"
}"#;

        let mut buf = Cursor::new(Vec::<u8>::new());
        yaml2json.document_to_writer(input, buf.get_mut()).unwrap();

        let res = String::from_utf8(buf.into_inner()).unwrap();
        assert_eq!(expected, res);
    }
}
