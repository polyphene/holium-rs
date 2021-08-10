

use crate::data::import::importer::Importer;
use std::path::Path;
use std::fs::File;
use anyhow::Context;
use anyhow::Result;
use serde_cbor;
use holium::data::importer::Importable;
use crate::data::DataError;

pub(crate) struct Cbor();

impl Importer for Cbor {
    fn import(&self, p: &Path) -> Result<Box<dyn Importable>> {
        let file = File::open(p)
            .context(DataError::FailedToOpenImportFile)?;
        let value: serde_cbor::Value = serde_cbor::from_reader(file)
            .context(DataError::FailedToParseImportFile)?;
        Ok(Box::new(value))
    }
}
