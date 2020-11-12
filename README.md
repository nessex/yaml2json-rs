# yaml2json

Because one more implementation can't hurt...

This implementation relies entirely on [Serde](https://github.com/serde-rs/serde) to provide fast and correct serialization and deserialization. In naïve tests, this provided a significant speed improvement over other implementations, though your mileage may vary.

## Installation
```
cargo install --git https://github.com/Nessex/yaml2json.git yaml2json
```

## Usage
```
Converts YAML to JSON

USAGE:
    yaml2json [FLAGS] <file>...

FLAGS:
    -h, --help       Prints help information
    -p, --pretty     
    -V, --version    Prints version information

ARGS:
    <file>...    Specify the path to files you want to convert
```

## Known Issues

 - Doesn't support YAML files with multiple documents in the same file