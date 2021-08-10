//! Module responsible of the CLI data [ Importer ] trait

use std::path::Path;

use anyhow::Result;

pub(crate) use bin::Bin;
pub(crate) use cbor::Cbor;
pub(crate) use json::Json;
pub(crate) use csv_importer::Csv;
use holium::data::importer::Importable;

mod json;
mod cbor;
mod bin;
mod csv_importer;

/// Trait interfacing files with Importable types from the holium API.
pub(crate) trait Importer {
    /// Parse a file into an [ Importable ] type.
    fn import(&self, p: &Path) -> Result<Box<dyn Importable>>;
}