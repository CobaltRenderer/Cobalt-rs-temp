// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

/// Standard result type for the Cobalt Renderer
pub type RendererResult<T> = std::result::Result<T, RendererError>;

/// Standard error kind for the Cobalt Renderer
///
/// In most instances the error kind will be `Failure` as the
/// C++ renderer does not return error types itself, instead most error
/// causes are reported in the log (see documentation for more)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RendererErrorKind {
    RendererError,
    LoadLibraryError,
    UnsupportedWindow,
    IoError,
}

/// Standard error type for the Cobalt Renderer
///
/// Errors have a kind and optionally a source error
pub struct RendererError {
    kind: RendererErrorKind,
    error: Option<Box<dyn std::error::Error>>,
}

impl RendererError {
    /// Create a new [`RendererError`] without a source error
    pub fn new(kind: RendererErrorKind) -> RendererError {
        RendererError { kind, error: None }
    }

    /// Create a new [`RendererError`] with a source error
    pub fn new_with_error(
        kind: RendererErrorKind,
        error: Box<dyn std::error::Error>,
    ) -> RendererError {
        RendererError {
            kind,
            error: Some(error),
        }
    }

    /// Get error kind
    pub fn kind(&self) -> RendererErrorKind {
        self.kind
    }

    /// Get a reference to the source error
    pub fn error(&self) -> Option<&dyn std::error::Error> {
        self.error.as_ref().map(|e| e.as_ref())
    }
}

/// Check the result of an expression and immediately return a [`RendererError`]
/// if not successful.
macro_rules! return_on_failure {
    ($code:expr) => {{
        let result: i32 = $code;
        if result != cobalt_renderer_sys::COBALT_SUCCESS {
            return Err(crate::RendererError::new(
                crate::RendererErrorKind::RendererError,
            ));
        }
    }};
}

impl From<std::io::Error> for RendererError {
    fn from(error: std::io::Error) -> Self {
        RendererError::new_with_error(RendererErrorKind::IoError, Box::new(error))
    }
}

impl std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.error {
            None => write!(f, "{:?}", self.kind),
            Some(e) => write!(f, "{:?}, {}", self.kind, e),
        }
    }
}

impl std::fmt::Debug for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
