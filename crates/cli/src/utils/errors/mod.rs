#[derive(thiserror::Error, Debug)]
/// Util errors for the CLI.
pub(crate) enum Error {
    /// This error is thrown when the value of an argument marked as 'required' still seems missing.
    #[error("missing value for required argument: {0}")]
    MissingRequiredArgument(String),
    /// This error is thrown when an object was expected to be found in store with a given key, but
    /// does not exist.
    #[error("missing object for key: {0}")]
    NoObjectForGivenKey(String),
    /// This error is thrown when an object was expected to be found in store with a given key, but
    /// does not exist.
    #[error("object already exists with key: {0}")]
    ObjectAlreadyExistsForGivenKey(String),
    /// This error is thrown when an object fails to be deserialized after being stored.
    #[error("failed to deserialize object")]
    BinCodeDeserializeFailed,
    /// This error is thrown when an object fails to be serialized before being stored.
    #[error("failed to serialize object")]
    BinCodeSerializeFailed,
    /// This error is thrown when an operation on the local database fails (first part of a
    /// CompareAndSwapResult error for instance).
    #[error("failed to operate on local database")]
    DbOperationFailed,
}