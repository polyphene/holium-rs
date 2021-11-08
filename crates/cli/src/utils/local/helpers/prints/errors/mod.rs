
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Structure creation error.
    #[error("cannot create {0} with name: {1}")]
    StructureCreationError(String, String),
}
