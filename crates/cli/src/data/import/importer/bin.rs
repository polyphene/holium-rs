use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;

use holium::data::importer::bin::BinaryValue;
use holium::data::importer::Importable;

use crate::data::DataError;
use crate::data::import::importer::Importer;

pub(crate) struct Bin();

impl Importer for Bin {
    fn import(&self, p: &Path) -> Result<Box<dyn Importable>> {
        let buf = fs::read(p)
            .context(DataError::FailedToReadImportFile)?;
        Ok(Box::new(BinaryValue::new(buf)))
    }
}
