# yaml2json-rs

yaml2json-rs converts one or more YAML documents into a JSON stream.

There are many implementations of this idea, with various limitations. This one aims for:
 - Speed
 - Multi-document support
 - Error control
     - selectively silence errors or output errors as JSON

This implementation relies heavily on the existing work in [Serde](https://github.com/serde-rs/serde), [yaml-rust](https://github.com/chyh1990/yaml-rust) and others to provide fast and correct serialization and deserialization. In naïve tests, this provided a significant speed improvement over other implementations, though your mileage may vary.

## Installation

Download pre-compiled binaries from the [Releases Page](https://github.com/Nessex/yaml2json-rs/releases/).

Or, install via `cargo`:
```
cargo install yaml2json-rs-bin --bin yaml2json
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

| crate | links | description |
| --- | --- | --- |
| yaml2json-rs-bin |  | A command line utility to convert YAML to JSON |
| yaml2json-rs | [![yaml2json-rs docs](https://docs.rs/yaml2json-rs/badge.svg)](https://docs.rs/yaml2json-rs/) | A library wrapping [serde-yaml](https://github.com/dtolnay/serde-yaml) and [serde-json](https://github.com/serde-rs/json) to convert a single YAML document to JSON |
| yaml-split | [![yaml2json-rs docs](https://docs.rs/yaml-split/badge.svg)](https://docs.rs/yaml-split/) | A library providing an iterator over individual YAML documents within a YAML file or stream |

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Support

Please consider supporting the amazing libraries that make this work:

 * [serde](https://github.com/serde-rs/serde)
 * [serde-yaml](https://github.com/dtolnay/serde-yaml)
 * [serde-json](https://github.com/serde-rs/json)
 * [anyhow](https://github.com/dtolnay/anyhow)
 * [thiserror](https://github.com/dtolnay/thiserror)
 * [clap](https://github.com/clap-rs/clap)

This project is not affiliated with the projects above, however they are entirely responsible for any successes this project has. Failures are our own.
 