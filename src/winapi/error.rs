use core::fmt;

use thiserror::Error;
use windows_sys::Win32::Foundation::GetLastError;

use crate::winapi::error_msg::get_error_as_string;

#[derive(Error, Debug)]
pub enum WinapiError {
    #[error("{0}")]
    WinapiCallError(#[from] WinapiCallError),
    #[error("{0}")]
    NullError(#[from] std::ffi::c_str::NulError),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
}
#[derive(Error, Debug)]
pub struct WinapiCallError {
    api_name: String,
    error_code: u32,
}

impl WinapiCallError {
    pub(crate) fn new(api_call: &str) -> Self {
        Self { api_name: api_call.to_string(), error_code: unsafe { GetLastError() } }
    }
}

impl fmt::Display for WinapiCallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} error. Code: 0x{:08x}, msg: {}",
            self.api_name,
            self.error_code,
            get_error_as_string(self.error_code).unwrap_or_default()
        )
    }
}
