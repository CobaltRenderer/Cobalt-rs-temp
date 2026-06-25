// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use num_enum::FromPrimitive;

use crate::renderer::{DeviceEnumerationFlags, GraphicsDeviceEnumerator};
use crate::{LibraryInternal, RendererResult};

use std::sync::Arc;

use cobalt_renderer_sys as sys;

/// Graphics API family a plugin uses
#[repr(i32)]
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApiFamily {
    OpenGl = sys::Cobalt_ApiFamily_OpenGL as i32,
    OpenGles = sys::Cobalt_ApiFamily_OpenGLES as i32,
    Direct3d = sys::Cobalt_ApiFamily_Direct3D as i32,
    Vulkan = sys::Cobalt_ApiFamily_Vulkan as i32,
    Metal = sys::Cobalt_ApiFamily_Metal as i32,
    #[num_enum(default)]
    Unknown
}

impl std::fmt::Display for ApiFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let family = match self {
            Self::OpenGl => "OpenGL",
            Self::OpenGles => "OpenGLES",
            Self::Direct3d => "Direct3D",
            Self::Vulkan => "Vulkan",
            Self::Metal => "Metal",
            Self::Unknown => "Unknown",
        };
        write!(f, "{}", family)
    }
}

/// Graphics API version a plugin uses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl Ord for ApiVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.major == other.major {
            self.minor.cmp(&other.minor)
        } else {
            self.major.cmp(&other.major)
        }
    }
}

impl PartialOrd for ApiVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Represents a loaded renderer plugin and entry point
/// for creating a [`GraphicsDeviceEnumerator`]
pub struct RendererPlugin {
    pub(crate) internal: Arc<RendererPluginInternal>,
    pub(crate) handle: sys::Cobalt_RendererInfo,
}

/// Actual plugin handle, shared between many objects via `Arc<T>`
/// to keep plugin object alive
///
/// Holds onto the module handle for the loaded DLL/SO
pub(crate) struct RendererPluginInternal {
    #[cfg(target_family = "windows")]
    _module: Arc<libloading::os::windows::Library>,
    #[cfg(target_family = "unix")]
    _module: Arc<libloading::os::unix::Library>,
    pub(crate) _library: Arc<LibraryInternal>,
}

impl RendererPlugin {
    #[cfg(target_family = "windows")]
    pub(crate) fn new(
        module: Arc<libloading::os::windows::Library>,
        handle: sys::Cobalt_RendererInfo,
        library: Arc<LibraryInternal>,
    ) -> Self {
        RendererPlugin {
            handle,
            internal: Arc::new(RendererPluginInternal { _module: module, _library: library }),
        }
    }

    #[cfg(target_family = "unix")]
    pub(crate) fn new(
        module: Arc<libloading::os::unix::Library>,
        handle: sys::Cobalt_RendererInfo,
        library: Arc<LibraryInternal>,
    ) -> Self {
        RendererPlugin {
            handle,
            plugin: Arc::new(RendererPluginInternal { module, library }),
        }
    }

    pub fn api_family(&self) -> ApiFamily {
        let value = unsafe { sys::Cobalt_RendererInfo_GetApiFamily(self.handle) };
        ApiFamily::from_primitive(value as i32)
    }

    pub fn target_api_version(&self) -> ApiVersion {
        let mut version = sys::Cobalt_Version::default();
        unsafe {
            sys::Cobalt_RendererInfo_GetTargetApiVersion(self.handle, &mut version);
        }
        ApiVersion {
            major: version.major,
            minor: version.minor,
        }
    }

    pub fn name(&self) -> String {
        // We don't know the size of the string.
        // Allocate what should be enough space and if it fills up
        // we will fetch the name again but with enough space
        let mut capacity: usize = 128;
        loop {
            let mut name: Vec<u8> = vec![0; capacity];

            let mut length = name.len();
            unsafe {
                // c_char will be either i8 or u8, so safe to treat u8 as c_char
                sys::Cobalt_RendererInfo_GetName(
                    self.handle,
                    name.as_mut_ptr() as *mut std::ffi::c_char,
                    &mut length,
                );
            }
            if length > name.len() {
                capacity = length;
                continue;
            }

            name.truncate(length);
            return String::from_utf8_lossy(name.as_slice()).to_string();
        }
    }

    pub fn display_name(&self) -> String {
        // We don't know the size of the string.
        // Allocate what should be enough space and if it fills up
        // we will fetch the name again but with enough space
        let mut capacity: usize = 128;
        loop {
            let mut name: Vec<u8> = vec![0; capacity];

            let mut length = name.len();
            unsafe {
                // c_char will be either i8 or u8, so safe to treat u8 as c_char
                sys::Cobalt_RendererInfo_GetDisplayName(
                    self.handle,
                    name.as_mut_ptr() as *mut std::ffi::c_char,
                    &mut length,
                );
            }
            if length > name.len() {
                capacity = length;
                continue;
            }

            name.truncate(length);
            return String::from_utf8_lossy(name.as_slice()).to_string();
        }
    }

    // Create a device enumerator, which finds graphics devices
    // for rendering
    pub fn create_device_enumerator(
        &mut self,
        flags: DeviceEnumerationFlags,
    ) -> RendererResult<GraphicsDeviceEnumerator> {
        let mut enumerator = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_RendererInfo_CreateGraphicsDeviceEnumerator(self.handle, &mut enumerator);
        }
        GraphicsDeviceEnumerator::new_and_enumerate_devices(
            enumerator,
            flags.bits() as sys::Cobalt_DeviceEnumerationFlags,
            self.internal.clone(),
        )
    }
}

impl Drop for RendererPluginInternal {
    fn drop(&mut self) {
        log::debug!("Unloading renderer plugin");
    }
}

unsafe impl Send for RendererPlugin {}
unsafe impl Sync for RendererPlugin {}
