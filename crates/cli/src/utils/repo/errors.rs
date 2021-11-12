#[derive(thiserror::Error, Debug)]
/// Errors for the interplanetary utility module.
pub(crate) enum Error {
    /// This error is thrown when a command that should only be run inside a Holium repository is ran
    /// outside of any repository.
    #[error("this command can only be run inside a Holium repository")]
    OutsideHoliumRepo,
}
