#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate clap;

use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

use clap::{App, Arg};

use std::error::Error;
use yaml2json_rs::{Style, Yaml2Json};
use yaml_split::DocumentIterator;

enum ErrorStyle {
    SILENT,
    STDERR,
    JSON,
}

impl FromStr for ErrorStyle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s {
            "silent" => Ok(ErrorStyle::SILENT),
            "stderr" => Ok(ErrorStyle::STDERR),
            "json" => Ok(ErrorStyle::JSON),
            _ => bail!("not a valid ErrorStyle"),
        }
    }
}

impl ToString for ErrorStyle {
    fn to_string(&self) -> String {
        match self {
            ErrorStyle::SILENT => "silent",
            ErrorStyle::STDERR => "stderr",
            ErrorStyle::JSON => "json",
        }
        .to_string()
    }
}

/// `ErrorPrinter` allows you to configure how errors will be printed.
struct ErrorPrinter {
    print_style: ErrorStyle,
}

impl ErrorPrinter {
    fn new(print_style: ErrorStyle) -> Self {
        Self { print_style }
    }

    fn print(&self, err: impl Error) {
        match self.print_style {
            ErrorStyle::SILENT => {}
            ErrorStyle::STDERR => eprintln!("{}", err),
            ErrorStyle::JSON => println!("{{\"yaml-error\":\"{}\"}}", err),
        };
    }

    fn print_string(&self, s: String) {
        match self.print_style {
            ErrorStyle::SILENT => {}
            ErrorStyle::STDERR => eprintln!("{}", s),
            ErrorStyle::JSON => println!("{{\"yaml-error\":\"{}\"}}", s),
        };
    }
}

fn write(yaml2json: &Yaml2Json, ep: &ErrorPrinter, read: impl Read) {
    let doc_iter = DocumentIterator::new(read);
    let mut first = true;
    let mut stdout = io::stdout();

    for res in doc_iter {
        if first {
            first = false;
        } else {
            match stdout.write(b"\n") {
                Ok(_) => {}
                Err(e) => ep.print(e),
            };
        }

        match res {
            Ok(doc) => yaml2json
                .document_to_writer(doc, &mut stdout)
                .unwrap_or_else(|e| ep.print(e)),
            Err(e) => ep.print(e),
        }
    }

    // Add final newline
    match stdout.write(b"\n") {
        Ok(_) => {}
        Err(e) => ep.print(e),
    };
}

fn main() {
    let default_err_style = ErrorStyle::STDERR.to_string();
    let usage = r#"./yaml2json file1.yaml file2.yaml

    cat file1.yaml | ./yaml2json

    ./yaml2json --error=json file1.yaml | jq"#;
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .usage(usage)
        .arg(
            Arg::with_name("pretty")
                .takes_value(false)
                .short("p")
                .long("pretty")
        )
        .arg(
            Arg::with_name("error")
                .takes_value(true)
                .short("e")
                .long("error")
                .default_value(default_err_style.as_str())
                .possible_value(ErrorStyle::SILENT.to_string().as_str())
                .possible_value(ErrorStyle::STDERR.to_string().as_str())
                .possible_value(ErrorStyle::JSON.to_string().as_str())
        )
        .arg(
            Arg::with_name("file")
                .help("Specify the path to files you want to convert. You can also pass files via stdin instead.")
                .multiple(true)
        )
        .get_matches();

    let fileopt = matches.values_of("file");
    let pretty = matches.is_present("pretty");
    let error = matches.value_of("error").unwrap();

    let ep = ErrorPrinter::new(ErrorStyle::from_str(error).unwrap());
    let yaml2json_style = if pretty {
        Style::PRETTY
    } else {
        Style::COMPACT
    };
    let yaml2json = Yaml2Json::new(yaml2json_style);

    // if: files are provided as arguments, read those instead of stdin
    if let Some(files) = fileopt {
        for f in files {
            let path = Path::new(f);

            if !path.exists() {
                ep.print_string(format!(
                    "file {} does not exist",
                    path.display().to_string()
                ));
            } else if path.is_dir() {
                ep.print_string(format!("{} is a directory", path.display().to_string()))
            } else {
                let file = File::open(f);

                match file {
                    Ok(f) => {
                        write(&yaml2json, &ep, f);
                    }
                    Err(e) => ep.print(e),
                }
            }
        }
    // else: No files provided as args, use stdin for input
    } else {
        let stdin = io::stdin();
        let stdin_lock = stdin.lock();

        write(&yaml2json, &ep, stdin_lock);
    }
}
