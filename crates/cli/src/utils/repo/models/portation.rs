use std::borrow::Borrow;
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::arg_enum;
use humansize::{file_size_opts, FileSize};
use prettytable::{cell, row, Row, Table};
use serde::{Deserialize, Serialize};
use serde_yaml;

use crate::utils::errors::Error::BinCodeSerializeFailed;
use crate::utils::local::helpers::prints::json::shorten_prettify_json_literal;
use crate::utils::local::helpers::prints::printable_model::PrintableModel;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Portation {
    #[serde(skip)]
    pub id: String,
    pub file_path: String,
    pub file_format: PortationFileFormat,
}

arg_enum! {
    #[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
    pub enum PortationFileFormat {
        bin,
        cbor,
        csv,
        json,
    }
}

/// Type serialized and stored in the configuration file.
type PortationSet = HashMap<String, Portation>;

/// Type used in the context handler.
pub struct Portations {
    pub path: PathBuf,
    pub set: PortationSet,
}

impl Portations {
    /// Create a [Portations] handler from the path of a portations configuration file.
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let file =
            File::open(&path).context(anyhow!("failed to open portations configuration file"))?;
        let metadata = file.metadata().context(anyhow!(
            "failed to read metadata of the portations configuration file"
        ))?;
        let mut set: PortationSet = if 0 < metadata.len() {
            serde_yaml::from_reader(file)
                .context(anyhow!("invalid portations configuration file"))?
        } else {
            PortationSet::new()
        };
        for (id, portation) in set.iter_mut() {
            portation.id = id.clone();
        }
        Ok(Portations { path, set })
    }

    /// Store the current set of portations to the configuration file.
    fn save(&self) -> Result<()> {
        let file = File::create(&self.path)
            .context(anyhow!("failed to create portations configuration file"))?;
        serde_yaml::to_writer(&file, &self.set)
            .context(anyhow!("failed to write portations configuration file"))
    }

    pub fn contains_key(&self, k: &String) -> bool {
        self.set.contains_key(k)
    }

    pub fn get(&self, k: &String) -> Option<&Portation> {
        self.set.get(k)
    }

    pub fn values(&self) -> Values<'_, String, Portation> {
        self.set.values()
    }

    pub fn insert(&mut self, k: String, v: Portation) -> Result<Option<Portation>> {
        let ok_res = self.set.insert(k, v);
        self.save().map(|_| ok_res)
    }

    pub fn remove(&mut self, k: &String) -> Result<Option<Portation>> {
        let ok_res = self.set.remove(k);
        self.save().map(|_| ok_res)
    }
}

impl PrintableModel for Portation {
    fn title_row() -> Row {
        row![
            b->"ID",
            "FILE FORMAT",
            "FILE PATH",
        ]
    }

    fn object_to_row(&self) -> Row {
        row![
            b->self.id,
            self.file_format,
            self.file_path,
        ]
    }
}
