use std::fmt;
use wasmer::{CompileError, ExportError, InstantiationError, RuntimeError};
use wasmer_runtime::error::{CacheError, CallError};

/*****************************************
 * Errors
 *****************************************/
#[derive(Debug)]
pub enum HoliumRuntimeError {
    WasmerCallError(CallError),
    WasmerCacheError(CacheError),
    WasmerRuntimeError(RuntimeError),
    WasmerExportError(ExportError),
    WasmerInstantiationError(InstantiationError),
    WasmerCompileError(CompileError),
    WasmerWatToWasmError(wat::Error),
}

impl From<CallError> for HoliumRuntimeError {
    fn from(error: CallError) -> HoliumRuntimeError {
        HoliumRuntimeError::WasmerCallError(error)
    }
}

impl From<CacheError> for HoliumRuntimeError {
    fn from(error: CacheError) -> HoliumRuntimeError {
        HoliumRuntimeError::WasmerCacheError(error)
    }
}

impl From<RuntimeError> for HoliumRuntimeError {
    fn from(error: RuntimeError) -> HoliumRuntimeError {
        HoliumRuntimeError::WasmerRuntimeError(error)
    }
}

impl From<ExportError> for HoliumRuntimeError {
    fn from(error: ExportError) -> HoliumRuntimeError {
        HoliumRuntimeError::WasmerExportError(error)
    }
}

impl From<InstantiationError> for HoliumRuntimeError {
    fn from(error: InstantiationError) -> HoliumRuntimeError {
        HoliumRuntimeError::WasmerInstantiationError(error)
    }
}

impl From<CompileError> for HoliumRuntimeError {
    fn from(error: CompileError) -> HoliumRuntimeError {
        HoliumRuntimeError::WasmerCompileError(error)
    }
}

impl From<wat::Error> for HoliumRuntimeError {
    fn from(error: wat::Error) -> HoliumRuntimeError {
        HoliumRuntimeError::WasmerWatToWasmError(error)
    }
}

impl fmt::Display for HoliumRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HoliumRuntimeError::WasmerCallError(inner) => write!(f, "{}", inner),
            HoliumRuntimeError::WasmerCacheError(inner) => write!(f, "{:?}", inner),
            HoliumRuntimeError::WasmerRuntimeError(inner) => write!(f, "{}", inner),
            HoliumRuntimeError::WasmerExportError(inner) => write!(f, "{}", inner),
            HoliumRuntimeError::WasmerInstantiationError(inner) => write!(f, "{}", inner),
            HoliumRuntimeError::WasmerCompileError(inner) => write!(f, "{}", inner),
            HoliumRuntimeError::WasmerWatToWasmError(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for HoliumRuntimeError {}
