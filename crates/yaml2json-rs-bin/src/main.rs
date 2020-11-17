#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate clap;

use std::fs::File;
use std::{io, process};
use std::io::{Read, Stdout, Stderr};
use std::path::Path;
use std::str::FromStr;

use clap::{App, Arg};

use std::error::Error;
use yaml2json_rs::{Style, Yaml2Json};
use yaml_split::{DocumentIterator, YamlSplitError};

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
    pretty: bool,
    print_style: ErrorStyle,
    stdout: Stdout,
    stderr: Stderr,
}

impl ErrorPrinter {
    fn new(print_style: ErrorStyle, pretty: bool) -> Self {
        Self {
            pretty,
            print_style,
            stdout: io::stdout(),
            stderr: io::stderr(),
        }
    }

    fn print(&mut self, err: impl Error) {
        self.print_string(err.to_string());
    }

    fn print_string(&mut self, s: String) {
        match self.print_style {
            ErrorStyle::SILENT => {}
            ErrorStyle::STDERR => write_or_exit(&mut self.stderr, format!("{}\n", s).as_str()),
            ErrorStyle::JSON => {
                let s = if self.pretty {
                    format!("{{\n  \"yaml-error\": \"{}\"\n}}\n", s)
                } else {
                    format!("{{\"yaml-error\":\"{}\"}}\n", s)
                };
                write_or_exit(&mut self.stdout, s.as_str());
            },
        };
    }
}

// `write_or_exit` is used for writing to stdout / stderr
// as otherwise the program may panic.
// As this program's entire purpose is to write data to stdout / stderr
// any failure here means we should just exit cleanly with an error code.
fn write_or_exit(io: &mut dyn io::Write, s: &str) {
    let w = io.write(s.as_bytes());

    if w.is_err() {
        process::exit(1);
    }
}

fn write(yaml2json: &Yaml2Json, ep: &mut ErrorPrinter, read: impl Read) {
    let doc_iter = DocumentIterator::new(read);
    let mut printed_last = false;
    let mut stdout = io::stdout();

    for res in doc_iter {
        // print a newline between regular output lines
        if printed_last {
            write_or_exit(&mut stdout, "\n");
        }

        match res {
            Ok(doc) => match yaml2json.document_to_writer(doc, &mut stdout) {
                Ok(_) => printed_last = true,
                Err(e) => {
                    printed_last = false;
                    ep.print(e);
                }
            },
            Err(e) => match e {
                // If there is an IOError, we should just exit.
                YamlSplitError::IOError(_) => process::exit(1),
            },
        }
    }

    if printed_last {
        // Add final newline
        write_or_exit(&mut stdout, "\n");
    }
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

    let mut ep = ErrorPrinter::new(ErrorStyle::from_str(error).unwrap(), pretty);
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
                        write(&yaml2json, &mut ep, f);
                    }
                    Err(e) => ep.print(e),
                }
            }
        }
    // else: No files provided as args, use stdin for input
    } else {
        let stdin = io::stdin();
        let stdin_lock = stdin.lock();

        write(&yaml2json, &mut ep, stdin_lock);
    }
}
