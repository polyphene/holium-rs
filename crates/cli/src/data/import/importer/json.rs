use std::fs::File;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use serde_json;

use holium::data::importer::Importable;

use crate::data::import::importer::Importer;
use crate::data::DataError;

pub(crate) struct Json();

impl Importer for Json {
    fn import(&self, p: &Path) -> Result<Box<dyn Importable>> {
        let file = File::open(p)
            .context(DataError::FailedToOpenImportFile)?;
        let value: serde_json::Value = serde_json::from_reader(file)
            .context(DataError::FailedToParseImportFile)?;
        Ok(Box::new(value))
    }
}
