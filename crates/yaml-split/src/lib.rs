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
    prepend_next: Option<String>,
}

impl <'a>DocumentIterator<'a> {
    pub fn new(reader: impl BufRead + 'a) -> DocumentIterator<'a> {
        DocumentIterator {
            reader: Box::new(reader),
            disambiguated: false,
            in_header: false,
            prepend_next: None,
        }
    }
}

impl Iterator for DocumentIterator<'_> {
    type Item = Result<String, YamlSplitError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: String;
        let mut current_file = match &self.prepend_next {
            Some(next) => String::new() + next.as_str(),
            None => String::new(),
        };
        self.prepend_next = None;

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
            if self.disambiguated {
                break;
            }

            // Empty the buffer. read_line appends, and we don't want that.
            buf = String::new();

            match self.reader.read_line(&mut buf) {
                Ok(l) => {
                    if l == 0 {
                        // We hit EOF already, and it's still not clear
                        // this file must have only whitespace, comments or be completely empty.
                        return None;
                    }

                    for c in buf.chars() {
                        match c {
                            ' ' | '\t' | '\r' => continue,
                            // # means this line is a comment, nothing to do.
                            // \n is a newline, also nothing to do, this line didn't
                            // tell us anything.
                            '#' | '\n' => break,
                            // % means this line is a directive, we must be in a header
                            '%' => {
                                self.disambiguated = true;
                                self.in_header = true;
                                break;
                            },
                            // anything else must mean we are in a bare document
                            _ => {
                                self.disambiguated = true;
                                self.in_header = false;
                                break;
                            }
                        };
                    }

                    // Append the current line to the document
                    current_file = String::new() + current_file.as_str() + buf.as_str();
                },
                Err(e) => {
                    return Some(Err(e.into()));
                }
            }
        }

        // Now that we know whether we are starting off in a directive or a document, we can
        // parse the rest of the YAML. In this loop we will look for the start and end of documents
        // as our YAML parser does not support parsing multiple documents at once.
        loop {
            buf = String::new();

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

                    if !self.in_header && directives_end {
                        // a new document has started already.
                        self.in_header = false;
                        // to not lose the current line, including any directives that might
                        // be on the line (after the "---"), we need to prepend it
                        // the next time someone calls next()
                        self.prepend_next = Some(buf);
                        return Some(Ok(current_file));
                    } else if end_of_doc {
                        // this document has ended, but we don't need this line.
                        // the next line must be a header, or "---"
                        self.in_header = true;
                        return Some(Ok(current_file));
                    } else if hit_eof {
                        // this document has ended, and nothing will follow.
                        return Some(Ok(current_file));
                    } else if self.in_header && directives_end {
                        self.in_header = false;
                    }

                    current_file = String::new() + current_file.as_str() + buf.as_str();
                }
                Err(e) => {
                    return Some(Err(e.into()));
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader};
    use crate::DocumentIterator;

    fn str_reader(s: &[u8]) -> BufReader<&[u8]> {
        BufReader::new(s)
    }

    #[test]
    fn bare_document() {
        let input = "abc: def";

        let reader = str_reader(input.as_bytes());
        let mut doc_iter = DocumentIterator::new(reader);

        let next = doc_iter.next().unwrap().unwrap();
        assert_eq!(next.as_str(), "abc: def");

        let fin = doc_iter.next().is_none();
        assert_eq!(true, fin);
    }

    #[test]
    fn document_with_header() {
        let input = r#"
---
abc: def
"#;

        let reader = str_reader(input.as_bytes());
        let mut doc_iter = DocumentIterator::new(reader);

        let next = doc_iter.next().unwrap().unwrap();
        assert_eq!(next.as_str(), r#"
---
abc: def
"#);

        let fin = doc_iter.next().is_none();
        assert_eq!(true, fin);
    }

    #[test]
    fn document_with_header_and_directive() {
        let input = r#"
%YAML 1.2
---
abc: def
"#;

        let reader = str_reader(input.as_bytes());
        let mut doc_iter = DocumentIterator::new(reader);

        let next = doc_iter.next().unwrap().unwrap();
        assert_eq!(next.as_str(), r#"
%YAML 1.2
---
abc: def
"#);

        let fin = doc_iter.next().is_none();
        assert_eq!(true, fin);
    }

    #[test]
    fn two_documents() {
        let input = r#"abc: def
---
aaa: bbb
"#;

        let reader = str_reader(input.as_bytes());
        let mut doc_iter = DocumentIterator::new(reader);

        let mut next = doc_iter.next().unwrap().unwrap();
        assert_eq!(next.as_str(), "abc: def\n");

        next = doc_iter.next().unwrap().unwrap();
        assert_eq!(next.as_str(), r#"---
aaa: bbb
"#);

        let fin = doc_iter.next().is_none();
        assert_eq!(true, fin);
    }

    #[test]
    fn two_documents_with_headers() {
        let input = r#"%YAML 1.2
---
abc: def
...

%YAML 1.2
---
aaa: bbb
"#;

        let reader = str_reader(input.as_bytes());
        let mut doc_iter = DocumentIterator::new(reader);

        let mut next = doc_iter.next().unwrap().unwrap();
        assert_eq!(next.as_str(), r#"%YAML 1.2
---
abc: def
"#);

        next = doc_iter.next().unwrap().unwrap();
        assert_eq!(next.as_str(), r#"
%YAML 1.2
---
aaa: bbb
"#);

        let fin = doc_iter.next().is_none();
        assert_eq!(true, fin);
    }

    #[test]
    fn document_medley() {
        let input = r#"%YAML 1.2
---
abc: def
---
%YAML: "not a real directive"
---
aaa: bbb
...
---
...
---
final: "document"
"#;

        let reader = str_reader(input.as_bytes());
        let mut doc_iter = DocumentIterator::new(reader);

        let mut next = doc_iter.next().unwrap().unwrap();
        assert_eq!(next.as_str(), r#"%YAML 1.2
---
abc: def
"#);

        next = doc_iter.next().unwrap().unwrap();
        assert_eq!(next.as_str(), r#"---
%YAML: "not a real directive"
"#);
        next = doc_iter.next().unwrap().unwrap();
        assert_eq!(next.as_str(), r#"---
aaa: bbb
"#      );

        next = doc_iter.next().unwrap().unwrap();
        assert_eq!(next.as_str(), r#"---
"#);

        next = doc_iter.next().unwrap().unwrap();
        assert_eq!(next.as_str(), r#"---
final: "document"
"#);

        let fin = doc_iter.next().is_none();
        assert_eq!(true, fin);
    }
}
