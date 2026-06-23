// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License
#[cfg(target_family = "windows")]
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::{ApiFamily, LibraryInternal, RendererError};
use crate::{RendererInfo, RendererResult};

use cobalt_renderer_sys as sys;

// Windows
#[cfg(target_family = "windows")]
const PLUGIN_EXTENSION:&str = "dll"; 
// macOS and iOS
#[cfg(all(target_family = "unix", any(target_os = "macos", target_os = "ios")))]
const PLUGIN_EXTENSION:&str = "dylib";
// Linux
#[cfg(all(target_family = "unix", not(any(target_os = "macos", target_os = "ios"))))]
const PLUGIN_EXTENSION:&str = "so";

// Preferred plugin preference list
// Plugin family and max API major version allowed

// Windows
// Prioritize Direct3D 11 due to stability and generally better performance
// Then Direct3D 12, Vulkan and OpenGL
#[cfg(target_family = "windows")]
const PLUGIN_PREFERENCE:[(ApiFamily, u32);4] = [
    (ApiFamily::Direct3d, 11),
    (ApiFamily::Direct3d, 12),
    (ApiFamily::Vulkan, u32::MAX),
    (ApiFamily::OpenGl, u32::MAX),
];
// macOS and iOS
// Prioritize Metal, then Vulkan (via MoltenVK), then OpenGL
#[cfg(all(target_family = "unix", any(target_os = "macos", target_os = "ios")))]
const PLUGIN_PREFERENCE:[(ApiFamily, u32);3] = [
    (ApiFamily::Metal, u32::MAX),
    (ApiFamily::Vulkan, u32::MAX),
    (ApiFamily::OpenGl, u32::MAX),
];
// macOS and iOS
// Prioritize Vulkan, then OpenGL
#[cfg(all(target_family = "unix", not(any(target_os = "macos", target_os = "ios"))))]
const PLUGIN_PREFERENCE:[(ApiFamily, u32);2] = [
    (ApiFamily::Vulkan, u32::MAX),
    (ApiFamily::OpenGl, u32::MAX),
];

pub struct RendererPluginEnumerator {
    plugins: Vec<RendererInfo>,
    library: Arc<LibraryInternal>,
}

impl RendererPluginEnumerator {
    pub(crate) fn new(library: Arc<LibraryInternal>) -> RendererPluginEnumerator {
        RendererPluginEnumerator { plugins:vec![], library }
    }

    pub fn enumerate_plugins_in_directory(&mut self, path: impl AsRef<Path>) -> RendererResult<()> {
        let path = path.as_ref();
        let dir_read = std::fs::read_dir(path)?;
        let mut found_plugins = Vec::with_capacity(5);
        for f in dir_read {
            let file_path = f?.path().canonicalize()?;
            if file_path.extension().is_some_and(|e| e == std::ffi::OsStr::new(PLUGIN_EXTENSION)) {
                found_plugins.push(file_path);
            }
        }

        for p in found_plugins {
            self.add_plugin_by_path(p)?;
        }

        Ok(())
    }

    pub fn add_plugin_by_path(&mut self, path: impl AsRef<Path>) -> RendererResult<()> {
        let path = path.as_ref().as_os_str();

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

        let mut index:u32 = 0;
        loop {
            let mut handle = std::ptr::null_mut();
            let result = unsafe {
                sys::Cobalt_GetRendererInfo(
                    self.library.handle,
                    lib_handle as *mut std::ffi::c_void,
                    index,
                    &mut handle
                )
            };
            match result {
                sys::COBALT_SUCCESS => {
                    let info = RendererInfo::new(library.clone(), handle, self.library.clone());
                    self.plugins.push(info);
                    index+=1;
                }
                _ => {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn all_plugins(self) -> Vec<RendererInfo> {
        self.plugins
    }

    pub fn all_plugins_ref(&self) -> &[RendererInfo] {
        self.plugins.as_slice()
    }

    pub fn preferred_plugin(mut self) -> Option<RendererInfo> {
        for pref in PLUGIN_PREFERENCE {
            // Store best plugin found, index into list and major version
            let mut best_plugin:Option<(usize, u32)> = None;
            for (p_index, p) in self.plugins.iter().enumerate() {
                let p_family = p.api_family();
                let p_version = p.target_api_version().major;
                if p_family == pref.0 && p_version <= pref.1 {
                    // Valid plugin, check it is a newer version of best found
                    match best_plugin {
                        None => best_plugin = Some((p_index, p_version)),
                        Some(best) => {
                            if best.1 < p_version {
                                best_plugin = Some((p_index, p_version));
                            }
                        }
                    }
                }
            }
            if let Some(best) = best_plugin {
                return Some(self.plugins.remove(best.0));
            }
        }
        
        // None preferred, return any plugin
        if self.plugins.is_empty() {
            Some(self.plugins.remove(0))
        } else {
            None
        }
    }
}
