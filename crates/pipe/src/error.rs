/// PipeError represents all error that might happened around `Pipe` processing
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum PipeError {
    #[error("Invalid mapping")]
    InvalidMappingError,
    #[error("Invalid mapping format")]
    InvalidMappingFormat,
    #[error("Error in serialization process")]
    SerializationError,
}
