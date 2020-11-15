use std::io;
use crate::Style::{COMPACT, PRETTY};
use thiserror::Error;
use core::fmt::Debug;

#[derive(Error, Debug)]
pub enum Yaml2JsonError {
    #[error(transparent)]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub enum Style {
    COMPACT,
    PRETTY,
}

pub struct Yaml2Json {
    style: Style,
}

impl Yaml2Json {
    pub fn new(style: Style) -> Self {
        Self {
            style,
        }
    }

    pub fn document_to_string(&self, document: String) -> Result<String, Yaml2JsonError> {
        let s: serde_json::Value = serde_yaml::from_str(document.as_str())?;

        let res = match self.style {
            COMPACT => serde_json::to_string(&s),
            PRETTY => serde_json::to_string_pretty(&s),
        };

        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into()),
        }
    }

    pub fn document_to_writer<W: io::Write>(&self, document: String, w: &mut W) -> Result<(), Yaml2JsonError> {
        let s: serde_json::Value = serde_yaml::from_str(document.as_str())?;

        let res = match self.style {
            PRETTY => serde_json::to_writer_pretty(w,&s),
            COMPACT => serde_json::to_writer(w,&s),
        };

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
