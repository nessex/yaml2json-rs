#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate clap;

use std::fs::File;
use std::io;
use std::io::{BufReader};
use std::path::Path;
use std::str::FromStr;

use clap::{App, Arg};

use yaml2json::{Style, Yaml2Json};
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
            "silent" | "none" => Ok(ErrorStyle::SILENT),
            "stderr" => Ok(ErrorStyle::STDERR),
            "json" => Ok(ErrorStyle::JSON),
            _ => bail!("not a valid ErrorStyle"),
        }
    }
}

struct ErrorPrinter {
    print_style: ErrorStyle,
}

impl ErrorPrinter {
    fn new(print_style: ErrorStyle) -> Self {
        Self {
            print_style,
        }
    }

    fn print(&self, err: String) {
        match self.print_style {
            ErrorStyle::SILENT => return,
            ErrorStyle::STDERR => eprintln!("{}", err),
            ErrorStyle::JSON => println!("{{\"yaml-error\":\"{}\"}}", err.to_string()),
        };
    }
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
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
                .default_value("stderr")
                .possible_value("stderr")
                .possible_value("none")
                .possible_value("json")
        )
        .arg(
            Arg::with_name("file")
                .help("Specify the path to files you want to convert")
                .multiple(true)
        )
        .get_matches();

    let fileopt = matches.values_of("file");
    let pretty = matches.is_present("pretty");
    let error = matches.value_of("error").unwrap();

    let eprinter = ErrorPrinter::new(ErrorStyle::from_str(error).unwrap());

    let yaml2json_style = if pretty { Style::PRETTY } else { Style::COMPACT };
    let yaml2json = Yaml2Json::new(yaml2json_style);

    let mut first = true;

    if let Some(files) = fileopt {
        for f in files {
            let path = Path::new(f);
            if path.exists() {
                let file = File::open(f).expect(format!("Cannot read file: {}", f).as_str());
                let buffered = BufReader::new(file);

                let doc_iter = DocumentIterator::new(buffered);

                for res in doc_iter {
                    match res {
                        Ok(doc) => match yaml2json.document_to_writer(doc, io::stdout()) {
                            Ok(_) => {}
                            Err(e) => eprinter.print(e.to_string()),
                        },
                        Err(e) => eprinter.print(e.to_string()),
                    };
                }
            } else {
                eprinter.print(format!("file {} does not exist", path.to_str().unwrap()));
            }
        }

        eprinter.print(format!("no valid input"))
    } else {
        let stdin = io::stdin();
        let stdin_lock = stdin.lock();

        let doc_iter = DocumentIterator::new(stdin_lock);
        for res in doc_iter {
            if first {
                first = false;
            } else {
                // Add newline
                eprintln!();
            }

            match res {
                Ok(doc) => match yaml2json.document_to_writer(doc, io::stdout()) {
                    Ok(_) => {}
                    Err(e) => eprinter.print(e.to_string()),
                },
                Err(e) => eprinter.print(e.to_string()),
            };
        }
    }

    // Add newline
    eprintln!();
}
