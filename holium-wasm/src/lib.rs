/*********************************************************
 * The holium-wasm module is the library that can be used in a Wasm runtime to interact with an host.
 * TODO As of now the host has to expose some functions. If not implemented this lib will not work. Any other way possible ?
 *********************************************************/

// TODO @PhilippeMts we used serde for the sake of development speed, should we think of something else
use serde::*;

#[allow(dead_code)]
pub(crate) mod utils;

/// ExecutionError represents all error that might have happened on the host
#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("Invalid storage key")]
    InvalidStorageKeyError,
    #[error("No content to set")]
    NoContentError,
    #[error("Out of memory")]
    OutOfMemoryError,
    #[error("Serialization error")]
    ExteralSerializationError,
    #[error("Unknown error")]
    UnknownError,
}

impl From<utils::Error> for ExecutionError {
    fn from(e: utils::Error) -> Self {
        match e {
            utils::Error::HoliumError(errno) => match errno {
                1 => ExecutionError::InvalidStorageKeyError,
                2 => ExecutionError::NoContentError,
                3 => ExecutionError::OutOfMemoryError,
                4 => ExecutionError::ExteralSerializationError,
                _ => ExecutionError::UnknownError,
            },
        }
    }
}

/// Function meant to be a generic serializer for any data that is supposed to be set on a storage
/// on the host.
pub fn set_payload<T>(storage_key: &str, payload: &T) -> Result<(), ExecutionError>
where
    T: ?Sized + Serialize,
{
    if storage_key.len() == 0 {
        return Err(ExecutionError::InvalidStorageKeyError);
    }

    let storage_key_slice = serde_json::to_vec(storage_key)?;
    let payload_slice = serde_json::to_vec(&payload)?;
    utils::set_payload(
        storage_key_slice.as_ptr(),
        storage_key_slice.len(),
        payload_slice.as_ptr(),
        payload_slice.len(),
    )?;

    Ok(())
}
