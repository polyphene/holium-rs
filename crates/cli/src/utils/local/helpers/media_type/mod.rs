use anyhow::Result;
use mime_guess;
use mime_guess::mime;
use thiserror;

use crate::utils::repo::models::portation::PortationFileFormat;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("file extension incompatible with selected file format")]
    IncompatibleMediaTypeAndPortationFormat,
}

/// Validate the coherence between a portation file path and its format, using media type guesses on
/// the path.
/// Returns an error in case of incoherence.
pub fn validate_mimetype_coherence(file_path: &str, file_format: &PortationFileFormat) -> Result<()> {
    let media_type_guess = mime_guess::from_path(file_path).first();
    match file_format {
        PortationFileFormat::bin => Ok(()),
        PortationFileFormat::cbor => {
            if let Some(media_type) = media_type_guess {
                if media_type.essence_str() == "application/cbor" { Ok(()) } else { Err(Error::IncompatibleMediaTypeAndPortationFormat.into()) }
            } else { Err(Error::IncompatibleMediaTypeAndPortationFormat.into()) }
        }
        PortationFileFormat::csv => {
            if media_type_guess == Some(mime::TEXT_CSV)
                || media_type_guess == Some(mime::TEXT_CSV_UTF_8) { Ok(()) } else { Err(Error::IncompatibleMediaTypeAndPortationFormat.into()) }
        }
        PortationFileFormat::json => {
            if media_type_guess == Some(mime::APPLICATION_JSON) { Ok(()) } else { Err(Error::IncompatibleMediaTypeAndPortationFormat.into()) }
        }
    }
}