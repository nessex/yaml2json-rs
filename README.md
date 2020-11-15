# yaml2json

yaml2json converts one or more YAML documents into a JSON stream.

There are many implementations of this idea, with various limitations. This one aims for:
 - Speed
 - Multi-document support
 - Error control
     - selectively silence errors or output errors as JSON

This implementation relies heavily on the existing work in [Serde](https://github.com/serde-rs/serde), [yaml-rust](https://github.com/chyh1990/yaml-rust) and others to provide fast and correct serialization and deserialization. In naïve tests, this provided a significant speed improvement over other implementations, though your mileage may vary.

## Installation
```
cargo install --git https://github.com/Nessex/yaml2json.git --bin yaml2json-bin yaml2json
```

## Usage
```
Utility to convert YAML files to JSON

USAGE:
    ./yaml2json file1.yaml file2.yaml

    cat file1.yaml | ./yaml2json

    ./yaml2json --error=json file1.yaml | jq

FLAGS:
    -h, --help       Prints help information
    -p, --pretty     
    -V, --version    Prints version information

OPTIONS:
    -e, --error <error>     [default: stderr]  [possible values: silent, stderr, json]

ARGS:
    <file>...    Specify the path to files you want to convert. You can also pass files via stdin instead.
```

## Crates

| crate | description |
| --- | --- |
| yaml2json | A library wrapping [serde-yaml](https://github.com/dtolnay/serde-yaml) and [serde-json](https://github.com/serde-rs/json) to convert a single YAML document to JSON |
| yaml2json-bin | A utility to convert YAML to JSON |
| yaml-split | A library providing an iterator over individual YAML documents within a YAML file or stream |
