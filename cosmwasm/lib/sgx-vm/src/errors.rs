use std::fmt::Debug;
use std::io;

use sgx_types::sgx_status_t;

use snafu::Snafu;
// use wasmer_runtime_core::cache::Error as CacheError;
// use wasmer_runtime_core::error as core_error;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {

    GenericErr {
        msg: String
    },

    #[snafu(display("Cache error: {}", msg))]
    CacheErr {
        msg: String,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    /*
    #[snafu(display("Compiling wasm: {}", source))]
    CompileErr {
        source: core_error::CompileError,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    */
    #[snafu(display("Filesystem error: {}", source))]
    IoErr {
        source: io::Error,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Hash doesn't match stored data"))]
    IntegrityErr {
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Parse error: {}", source))]
    ParseErr {
        source: serde_json_wasm::de::Error,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Serialize error: {}", source))]
    SerializeErr {
        source: serde_json_wasm::ser::Error,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    /*
    #[snafu(display("Resolving wasm function: {}", source))]
    ResolveErr {
        source: core_error::ResolveError,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Calling wasm function: {}", source))]
    RuntimeErr {
        source: core_error::RuntimeError,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    */
    #[snafu(display("Region too small. Got {}, required {}", size, required))]
    RegionTooSmallErr {
        size: usize,
        required: usize,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Validating: {}", msg))]
    ValidationErr {
        msg: String,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    /*
    #[snafu(display("Wasmer error: {}", source))]
    WasmerErr {
        source: core_error::Error,
        #[cfg(feature = "backtraces")]
        backtrace: snafu::Backtrace,
    },
    */
    #[snafu(display("Enclave error: {}", inner))]
    EnclaveErr {
        inner: enclave_ffi_types::EnclaveError,
    },
    CryptoErr {
        inner: enclave_ffi_types::CryptoError,
    },
    #[snafu(display("SDK error: {}", inner))]
    SdkErr { inner: sgx_status_t },

}

pub type Result<T, E = Error> = core::result::Result<T, E>;

/*
pub trait CacheExt<T: Debug> {
    fn convert_cache(self) -> Result<T>;
}

impl<T: Debug> CacheExt<T> for Result<T, CacheError> {
    fn convert_cache(self) -> Result<T> {
        self.map_err(|err| {
            let msg = format!("{:?}", err);
            // construct like this (not just Err(Error::CacheErr)) to allow backtraces
            let res: Result<T> = CacheErr { msg }.fail();
            res.unwrap_err()
        })
    }
}
*/