// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use cobalt_renderer_sys as sys;

use num_enum::FromPrimitive;

/// Standard result type for the Cobalt Renderer
pub type RendererResult<T> = std::result::Result<T, RendererError>;

/// Standard error type for the Cobalt Renderer
///
/// In most instances the error type will be `Failure` as the
/// C++ renderer does not return error types itself, instead most error
/// causes are reported in the log (see documentation for more)
#[derive(FromPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum RendererError {
    Failure = sys::COBALT_FAILURE,
    LoadLibraryError,
    InvalidLibraryError,
    FailedGetInfo,
    UnsupportedWindow,
    InvalidPath,
    IoError,
    #[num_enum(default)]
    UnknownError,
}

/// Check the result of an expression and immediately return a [`RendererError`]
/// if not successful.
macro_rules! return_on_failure {
    ($code:expr) => {{
        use num_enum::FromPrimitive;
        let result: i32 = $code;
        if result != sys::COBALT_SUCCESS {
            return Err(RendererError::from_primitive(result));
        }
    }};
}

impl From<std::io::Error> for RendererError {
    fn from(_value: std::io::Error) -> Self {
        // TODO(DTM): More comprehensive handling here
        Self::IoError
    }
}

impl std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Failure => write!(
                f,
                "An error occurred in the function call. Please see log output for reason. This is often caused by improper API usage."
            ),
            Self::LoadLibraryError => write!(
                f,
                "The renderer plugin could not be loaded. The system call 'LoadLibrary' (win32) or 'dlopen' (linux) failed. This may be due to the shared library file not existing at the specified path or the file not being a valid library file."
            ),
            Self::InvalidLibraryError => write!(
                f,
                "The renderer plugin is not valid. The function 'GetRendererPlugin' may not exist in the library."
            ),
            Self::FailedGetInfo => write!(
                f,
                "Failed to get renderer information. The call to 'GetRendererPlugin' returned an error."
            ),
            Self::UnsupportedWindow => write!(
                f,
                "The raw window handle provided is for an unsupported platform. Supported platforms are: Win32"
            ),
            Self::InvalidPath => write!(
                f,
                "The path provided was not valid unicode and cannot be used"
            ),
            Self::IoError => write!(f, "IO Error"),
            Self::UnknownError => write!(
                f,
                "An unknown error code was returned from the renderer plugin. This may indicate an out of date bindings ABI or a serious problem in the plugin."
            ),
        }
    }
}
