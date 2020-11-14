use std::io::{BufRead};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YamlSplitError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub struct DocumentIterator<'a> {
    reader: Box<dyn BufRead + 'a>,
    disambiguated: bool,
    in_header: bool,
}

impl <'a>DocumentIterator<'a> {
    pub fn new(reader: impl BufRead + 'a) -> DocumentIterator<'a> {
        DocumentIterator {
            reader: Box::new(reader),
            disambiguated: false,
            in_header: false,
        }
    }
}

impl Iterator for DocumentIterator<'_> {
    type Item = Result<String, YamlSplitError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_file = String::new();
        let mut buf = String::new();

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
        if !self.disambiguated {
            loop {
                match self.reader.read_line(&mut buf) {
                    Ok(l) => {
                        if l == 0 {
                            // We hit EOF already, and it's still not clear
                            // this file must have only whitespace, comments or be completely empty.
                            return None;
                        }

                        for c in buf.chars() {
                            match c {
                                ' ' | '\t' => continue,
                                // # means this line is a comment, nothing to do.
                                '#' => break,
                                // % means this line is a directive, we must be in a header
                                '%' => {
                                    self.disambiguated = true;
                                    self.in_header = true;
                                },
                                // anything else must mean we are in a bare document
                                _ => {
                                    self.disambiguated = true;
                                    self.in_header = false;
                                }
                            };
                        }

                        // Append the current line to the document
                        current_file = String::new() + current_file.as_str() + buf.as_str();

                        // Empty the buffer. Read_line appends, and we don't want that.
                        buf = String::new();

                        if self.disambiguated {
                            break;
                        }
                    },
                    Err(e) => {
                        return Some(Err(e.into()));
                    }
                }
            }
        }

        // Now that we know whether we are starting off in a directive or a document, we can
        // parse the rest of the YAML. In this loop we will look for the start and end of documents
        // as our YAML parser does not support parsing multiple documents at once.
        loop {
            match self.reader.read_line(&mut buf) {
                Ok(l) => {
                    let hit_eof = l == 0;
                    let cf_len = current_file.len();

                    // If there is absolutely nothing to do (i.e. the current file data is empty, and
                    // we're at EOF), just exit the loop.
                    if hit_eof && cf_len == 0 {
                        return None;
                    }

                    let end_of_doc = buf.starts_with("...");
                    let directives_end = buf.starts_with("---");

                    if hit_eof || end_of_doc || (!self.in_header && directives_end) {
                        self.in_header = true;
                        return Some(Ok(current_file));
                    }

                    current_file = String::new() + current_file.as_str() + buf.as_str();
                    buf = String::new();
                }
                Err(e) => {
                    return Some(Err(e.into()));
                }
            };
        }
    }
}
