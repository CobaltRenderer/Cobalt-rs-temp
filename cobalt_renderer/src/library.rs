// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License
use crate::{RendererInfo, RendererPluginEnumerator};
use crate::{RendererError, RendererResult};
use std::sync::Arc;

use cobalt_renderer_sys as sys;

pub struct Library {
    pub(crate) internal: Arc<LibraryInternal>,
}

pub(crate) struct LibraryInternal {
    pub(crate) handle: sys::Cobalt_Library,
}

impl Library {
    fn new(handle: sys::Cobalt_Library) -> Self {
        Library { internal: Arc::new(LibraryInternal { handle }) }
    }

    /// Handle callbacks from the renderer and log them
    extern "C" fn renderer_log_callback(
        severity: sys::Cobalt_LogSeverity,
        scope: *const std::ffi::c_char,
        scope_length: usize,
        message: *const std::ffi::c_char,
        message_length: usize,
    ) {
        let message = unsafe {
            match std::str::from_utf8(std::slice::from_raw_parts(
                message as *const u8,
                message_length,
            )) {
                Ok(s) => s,
                Err(_) => {
                    log::error!(target:"Cobalt", "Log message from Cobalt Renderer received, but not valid UTF-8");
                    return;
                }
            }
        };
        let scope = unsafe {
            match std::str::from_utf8(std::slice::from_raw_parts(scope as *const u8, scope_length))
            {
                Ok(s) => s,
                Err(_) => {
                    log::error!(target:"Cobalt", "Log scope from Cobalt Renderer received, but not valid UTF-8");
                    return;
                }
            }
        };

        match severity as sys::Cobalt_LogSeverity {
            sys::Cobalt_LogSeverity_Critical | sys::Cobalt_LogSeverity_Error => {
                log::error!(target:"Cobalt", "{scope}: {message}")
            }
            sys::Cobalt_LogSeverity_Warning => log::warn!(target:"Cobalt", "{scope}: {message}"),
            sys::Cobalt_LogSeverity_Debug => log::debug!(target:"Cobalt", "{scope}: {message}"),
            sys::Cobalt_LogSeverity_Trace => log::trace!(target:"Cobalt", "{scope}: {message}"),
            // Info or any other value
            _ => log::info!(target:"Cobalt", "{scope}: {message}"),
        }
    }

    /// Load a renderer plugin and get information about it.
    ///
    /// `lib_path` should be a path to a shared library on disk that contains the renderer plugin.
    /// This function will attempt to open the shared library and retrieve information about it.
    /// The shared library will be automatically unloaded when this object and any other
    /// derivate objects are dropped. Multiple plugins may be loaded at the same time
    /// (e.g you may want to see what is available and select the appropriate plugin).
    /// Loading the same renderer plugin multiple times should not be done
    pub fn load_renderer_plugin(
        &mut self,
        plugin_path: impl AsRef<std::path::Path>,
        index: Option<u32>,
    ) -> RendererResult<RendererInfo> {
        let path = plugin_path.as_ref().as_os_str();
        log::debug!("Loading renderer plugin '{}'", path.display());

        let library = unsafe {
            #[cfg(target_family = "windows")]
            {
                libloading::os::windows::Library::load_with_flags(
                    path,
                    libloading::os::windows::LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR
                        | libloading::os::windows::LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
                )
                .map_err(|e| {
                    log::error!("Failed to load library '{}', {e:?}", path.display());
                    RendererError::LoadLibraryError
                })?
            }
            #[cfg(target_family = "unix")]
            {
                libloading::os::unix::Library::new(path).map_err(|e| {
                    log::error!("Failed to load library '{}', {e:?}", path.display());
                    RendererError::LoadLibraryError
                })?
            }
        };

        // Work around to get raw handle for GetRendererInfo
        let lib_handle = library.into_raw();
        #[cfg(target_family = "windows")]
        let library = Arc::new(unsafe {
            libloading::os::windows::Library::from_raw(lib_handle)
        });
        #[cfg(target_family = "unix")]
        let library = Arc::new(unsafe {
            libloading::os::unix::Library::from_raw(lib_handle)
        });

        let mut handle = std::ptr::null_mut();
        unsafe {
            return_on_failure!(sys::Cobalt_GetRendererInfo(
                self.internal.handle,
                lib_handle as *mut std::ffi::c_void,
                index.unwrap_or_default(),
                &mut handle
            ))
        }

        Ok(RendererInfo::new(library, handle, self.internal.clone()))
    }

    pub fn renderer_plugin_enumerator(&self) -> RendererPluginEnumerator {
        RendererPluginEnumerator::new(self.internal.clone())
    }
}

impl Drop for LibraryInternal {
    fn drop(&mut self) {
        unsafe { sys::Cobalt_Terminate(self.handle) };
    }
}

pub fn init() -> RendererResult<Library> {
    let mut handle = std::ptr::null_mut();
    let level = match log::max_level() {
        log::LevelFilter::Off => sys::Cobalt_LogSeverityFilter_None,
        log::LevelFilter::Trace => sys::Cobalt_LogSeverityFilter_TraceOrHigher,
        log::LevelFilter::Debug => sys::Cobalt_LogSeverityFilter_DebugOrHigher,
        log::LevelFilter::Info => sys::Cobalt_LogSeverityFilter_InfoOrHigher,
        log::LevelFilter::Warn => sys::Cobalt_LogSeverityFilter_WarningOrHigher,
        log::LevelFilter::Error => sys::Cobalt_LogSeverityFilter_ErrorOrHigher,
    };
    unsafe {
        return_on_failure!(sys::Cobalt_Initialize(
            Some(Library::renderer_log_callback),
            level,
            &mut handle
        ))
    }
    Ok(Library::new(handle))
}

unsafe impl Send for LibraryInternal {}
unsafe impl Sync for LibraryInternal {}