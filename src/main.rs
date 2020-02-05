#[macro_use]
extern crate clap;
use clap::{Arg, App};
use std::fs::File;
use std::path::Path;
use serde::{Serialize};
use std::io;

fn main() {
    let matches = App::new("yaml2json")
        .version(crate_version!())
        .author("Nathan Essex <nathan@essex.id.au>")
        .about("Converts YAML to JSON")
        .arg(
            Arg::with_name("pretty")
                .takes_value(false)
                .short("p")
                .long("pretty")
        )
        .arg(
            Arg::with_name("file")
                .help("Specify the path to files you want to convert")
                .required(true)
                .multiple(true)
        )
        .get_matches();

    let files = matches.values_of("file").unwrap();
    let pretty = matches.is_present("pretty");

    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut pretty_serializer = serde_json::Serializer::with_formatter(io::stdout(), formatter);
    let mut compact_serializer = serde_json::Serializer::new(io::stdout());

    for f in files {
        let path = Path::new(f);
        if path.exists() {
            let file = File::open(f).expect("Cannot read file");
            let s: serde_json::Value = serde_yaml::from_reader(file).expect("Cannot convert YAML to JSON");

            if pretty {
                s.serialize(&mut pretty_serializer).unwrap();
            } else {
                s.serialize(&mut compact_serializer).unwrap();
            }

            // Add a newline
            println!();
        }
    }
}
