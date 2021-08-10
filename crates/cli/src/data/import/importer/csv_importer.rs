use serde_cbor::Value as CborValue;

use crate::data::import::importer::Importer;
use std::path::Path;
use std::fs::File;
use anyhow::Context;
use anyhow::Result;
use serde_cbor;
use holium::data::importer::Importable;
use crate::data::DataError;

use csv::StringRecord;

pub(crate) struct Csv();

impl Importer for Csv {
    fn import(&self, p: &Path) -> Result<Box<dyn Importable>> {
        let file = File::open(p)
            .context(DataError::FailedToOpenImportFile)?;
        let mut reader = csv::Reader::from_reader(file);
        let records: Vec<StringRecord> = reader.records()
            .into_iter()
            .filter_map(|record| record.ok())
            .collect();
        Ok(Box::new(records))
    }
}
