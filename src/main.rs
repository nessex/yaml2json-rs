#[macro_use]
extern crate clap;
use clap::{Arg, App};
use std::fs::File;
use std::path::Path;
use serde::{Serialize};
use std::io;
use std::io::{BufRead, BufReader};

fn parse_document_stream(mut br: impl BufRead, pretty: bool) {
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut pretty_serializer = serde_json::Serializer::with_formatter(io::stdout(), formatter);
    let mut compact_serializer = serde_json::Serializer::new(io::stdout());

    let mut current_file = String::new();
    let mut buf = String::new();
    let mut in_header = false;

    // First, we must disambiguate between a bare document and a directive at the top of the
    // file (before any directive end "---" markers). To do this, we must look for a #, % or
    // other non-whitespace character as the first character on a line:
    //
    // - # indicates a comment, the line will be ignored
    // - % indicates a directive, we should assume the rest of the header is also a directive as
    //    % is not a valid character at the start of a line, before a --- is seen.
    // - anything else indicates we must currently be looking at a bare document's content
    //
    // XXX: This loop also builds up buffers that are shared with the next loop. The reader
    // is also shared and so the next loop will start off where this one ends and assume the
    // buffers have the correct content.
    loop {
        match br.read_line(&mut buf) {
            Ok(l) => {
                if l == 0 {
                    // We hit EOF already, and it's still not clear
                    // this file must have only whitespace, comments or be completely empty.
                    return;
                }

                let mut disambiguated = false;

                for c in buf.chars() {
                    match c {
                        ' ' | '\t' => continue,
                        // # means this line is a comment, nothing to do.
                        '#' => break,
                        // % means this line is a directive, we must be in a header
                        '%' => {
                            disambiguated = true;
                            in_header = true;
                        },
                        // anything else must mean we are in a bare document
                        _ => {
                            disambiguated = true;
                            in_header = false;
                        }
                    };
                }

                // Append the current line to the document
                current_file = String::new() + current_file.as_str() + buf.as_str();

                // Empty the buffer. Read_line appends, and we don't want that.
                buf = String::new();

                if disambiguated {
                    break;
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
    }

    // Now that we know whether we are starting off in a directive or a document, we can
    // parse the rest of the YAML. In this loop we will look for the start and end of documents
    // as our YAML parser does not support parsing multiple documents at once.
    loop {
        match br.read_line(&mut buf) {
            Ok(l) => {
                let hit_eof = l == 0;
                let cf_len = current_file.len();

                // If there is absolutely nothing to do (i.e. the current file data is empty, and
                // we're at EOF), just exit the loop.
                if hit_eof && cf_len == 0 {
                    break;
                }

                let end_of_doc = buf.starts_with("...");
                let directives_end = buf.starts_with("---");

                if hit_eof || end_of_doc || (!in_header && directives_end) {
                    // We need to print out the JSON version of what we've got
                    let s: serde_json::Value = serde_yaml::from_str(current_file.as_str()).expect("Cannot convert YAML to JSON");

                    if pretty {
                        s.serialize(&mut pretty_serializer).unwrap();
                    } else {
                        s.serialize(&mut compact_serializer).unwrap();
                    }

                    // Add a newline
                    println!();
                }

                if hit_eof {
                    break;
                } else if end_of_doc || (!in_header && directives_end) {
                    in_header = true;
                    current_file = String::new();
                    buf = String::new();
                } else {
                    current_file = String::new() + current_file.as_str() + buf.as_str();
                    buf = String::new();
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
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
            Arg::with_name("file")
                .help("Specify the path to files you want to convert")
                .multiple(true)
        )
        .get_matches();

    let fileopt = matches.values_of("file");
    let pretty = matches.is_present("pretty");

    if let Some(files) = fileopt {
        for f in files {
            let path = Path::new(f);
            if path.exists() {
                let file = File::open(f).expect(format!("Cannot read file: {}", f).as_str());
                let buffered = BufReader::new(file);

                parse_document_stream(buffered, pretty);
            }
        }
    } else {
        let stdin = io::stdin();
        let stdin_lock = stdin.lock();

        parse_document_stream(stdin_lock, pretty);
    }
}
