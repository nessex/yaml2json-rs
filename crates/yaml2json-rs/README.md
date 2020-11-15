# yaml2json-rs

yaml2json-rs is a library which helps to convert YAML document strings to JSON. Output can be returned as a string, or passed on to anything that implements `io::Write`.

This library is a thin wrapper around [serde-yaml](https://github.com/dtolnay/serde-yaml) and [serde-json](https://github.com/serde-rs/json).

## Usage

```
let yaml = r#"
hello: world
"#;

let yaml2json = Yaml2Json::new(Style::PRETTY);
let json = yaml2json.document_to_string(yaml);

println!("{}", json);
```
