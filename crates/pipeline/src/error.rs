/// PipelineError represents all errors that might happened around `Pipeline` processing
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum PipelineError {
    #[error("CID not found in DAG")]
    CidNotFound(String),
    // TODO can be more verbose on connection error
    #[error("Connection error")]
    ConnectionError,
    #[error("Object is not a Pipe")]
    ObjectNotPipe(String),
}
