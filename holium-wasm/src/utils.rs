/*********************************************************
 * The utils module is the one communicating to functions exposed by the host. As the Wasm module
 * is only calling them by interface definition some parts are unsafe.
 *********************************************************/

use std::mem::MaybeUninit;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    HoliumError(i32),
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HoliumError(e) => write!(f, "Holium error {}", e),
        }
    }
}

pub type Char8 = u8;
pub type Char32 = u32;
pub type WrittenBytes = usize;
pub type HoliumPtr<T> = *const T;
pub type HoliumMutPtr<T> = *mut T;

pub type ExecutionError = Char32;
