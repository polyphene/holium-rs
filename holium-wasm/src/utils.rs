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

/// EXTERNAL_ERROR embodies the errors that could happen on the host while calling an external function
#[allow(non_snake_case)]
pub mod EXTERNAL_ERROR {
    use super::ExecutionError;
    pub const SUCCESS: ExecutionError = 0;
    pub const INVALID_STORAGE_KEY_ERROR: ExecutionError = 1;
    pub const NO_CONTENT_ERROR: ExecutionError = 2;
    pub const OUT_OF_MEMORY_ERROR: ExecutionError = 3;
    pub const SERIALIZATION_ERROR: ExecutionError = 4;
}

/// Function to interact with the external `set_payload` function. It will set a value in a temporary
/// storage inside the host memory.
///
/// As the host will read value directly from the linear memory both `storage_key` and `payload` are
/// passed by pointers and length.
pub fn set_payload(
    storage_key_ptr: HoliumPtr<u8>,
    storage_key_len: usize,
    payload_ptr: HoliumPtr<u8>,
    payload_len: usize,
) -> Result<(), Error> {
    extern "C" {
        fn set_payload(
            storage_key_ptr: HoliumPtr<u8>,
            storage_key_len: usize,
            payload_ptr: HoliumPtr<u8>,
            payload_len: usize,
        ) -> ExecutionError;
    }
    let res: u32 =
        unsafe { set_payload(storage_key_ptr, storage_key_len, payload_ptr, payload_len) };
    if res != 0 {
        return Err(Error::HoliumError(res as _));
    }
    Ok(())
}

/// Function to interact with the external `get_payload` function. It will get a value from a temporary
/// storage inside the host memory.
///
/// As the host will read value directly from the linear memory the `storage_key` is passed by
/// a pointer and a length. Pointer are also prepared for the payload and its length to be written on
/// by the host.
pub fn get_payload(
    storage_key_ptr: HoliumPtr<u8>,
    storage_key_len: usize,
    payload_buf_ptr: HoliumMutPtr<u8>,
) -> Result<WrittenBytes, Error> {
    extern "C" {
        fn get_payload(
            storage_key_ptr: HoliumPtr<u8>,
            storage_key_len: usize,
            payload_buf_ptr: HoliumMutPtr<Char8>,
            result_ptr: HoliumMutPtr<WrittenBytes>,
        ) -> ExecutionError;
    }

    let mut result_ptr: MaybeUninit<usize> = std::mem::MaybeUninit::uninit();
    let res: u32 = unsafe {
        get_payload(
            storage_key_ptr,
            storage_key_len,
            payload_buf_ptr,
            result_ptr.as_mut_ptr(),
        )
    };
    if res != 0 {
        return Err(Error::HoliumError(res as _));
    }

    Ok(unsafe { result_ptr.assume_init() })
}
