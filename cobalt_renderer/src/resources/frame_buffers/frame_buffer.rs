// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use std::ffi::c_void;
#[allow(unused)]
use std::num::NonZero;
#[allow(unused)]
use std::ptr::NonNull;
use std::sync::Arc;

#[cfg(feature = "raw_window_handle")]
use raw_window_handle::RawWindowHandle;

use bitflags::bitflags;

use super::FrameBufferOutput;
use crate::renderer::RendererInternal;
use crate::resources::textures::TextureBuffer2D;
use crate::{RendererError, RendererErrorKind, RendererResult};

use cobalt_renderer_sys as sys;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachmentType {
    Color = sys::Cobalt_AttachmentType_Color as i32,
    Depth = sys::Cobalt_AttachmentType_Depth as i32,
    Stencil = sys::Cobalt_AttachmentType_Stencil as i32,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowDepthStencilMode {
    None = sys::Cobalt_WindowDepthStencilMode_None as i32,
    DepthUNorm16 = sys::Cobalt_WindowDepthStencilMode_DepthUNorm16 as i32,
    DepthUNorm24 = sys::Cobalt_WindowDepthStencilMode_DepthUNorm24 as i32,
    DepthUNorm24StencilUInt8 = sys::Cobalt_WindowDepthStencilMode_DepthUNorm24StencilUInt8 as i32,
    DepthFloat32 = sys::Cobalt_WindowDepthStencilMode_DepthFloat32 as i32,
    DepthFloat32StencilUInt8 = sys::Cobalt_WindowDepthStencilMode_DepthFloat32StencilUInt8 as i32,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowColorSpaceMode {
    Default = sys::Cobalt_WindowColorSpaceMode_Default as i32,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WindowBindingFlags : i32 {
        const None = sys::Cobalt_WindowBindingFlags_None as i32;
        const LimitSwapToVSync = sys::Cobalt_WindowBindingFlags_LimitSwapToVSync as i32;
        const AllowTearing = sys::Cobalt_WindowBindingFlags_AllowTearing as i32;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Window {
    #[cfg(target_os = "windows")]
    Win32 {
        hinstance: NonZero<isize>,
        hwnd: NonZero<isize>,
    },
    #[cfg(target_os = "macos")]
    AppKit { view: NonNull<c_void> },
    #[cfg(target_os = "linux")]
    Wayland {
        display: NonNull<c_void>,
        surface: NonNull<c_void>,
    },
    #[cfg(target_os = "linux")]
    Xcb {
        connection: NonNull<c_void>,
        window: i32,
    },
    #[cfg(target_os = "linux")]
    Xlib {
        display: NonNull<c_void>,
        window: i32,
    },
}

impl Window {
    #[cfg(feature = "raw_window_handle")]
    #[allow(unused_variables)]
    pub fn new_from_raw_handles(
        display_handle: raw_window_handle::RawDisplayHandle,
        window_handle: raw_window_handle::RawWindowHandle,
    ) -> RendererResult<Self> {
        match window_handle {
            #[cfg(target_os = "windows")]
            RawWindowHandle::Win32(w) => {
                let hinstance = w.hinstance.ok_or_else(|| {
                    RendererError::new_with_error(
                        RendererErrorKind::UnsupportedWindow,
                        "Win32 window does not contain a required hinstance pointer".into(),
                    )
                })?;
                Ok(Self::Win32 {
                    hinstance,
                    hwnd: w.hwnd,
                })
            }
            #[cfg(target_os = "macos")]
            RawWindowHandle::AppKit(w) => Ok(Self::AppKit { view: w.ns_view }),
            #[cfg(target_os = "linux")]
            RawWindowHandle::Wayland(w) => {
                if let RawDisplayHandle::Wayland(d) = display_handle {
                    Ok(Self::Wayland {
                        display: d.display,
                        surface: w.surface,
                    })
                } else {
                    Err(RendererError::new_with_error(RendererErrorKind::UnsupportedWindow, "Display handle must be Wayland variant because window handle is Wayland variant".into()))
                }
            }
            #[cfg(target_os = "linux")]
            RawWindowHandle::Xcb(w) => {
                if let RawDisplayHandle::Xcb(d) = display_handle {
                    let connection = d.connection.ok_or_else(|| {
                        Err(RendererError::new_with_error(
                            RendererErrorKind::UnsupportedWindow,
                            "Xcb display does not contain a required display pointer".into(),
                        ))
                    })?;
                    Ok(Self::Xcb {
                        connection,
                        window: w.window.get() as i32,
                    })
                } else {
                    Err(RendererError::new_with_error(
                        RendererErrorKind::UnsupportedWindow,
                        "Display handle must be Xcb variant because window handle is Xcb variant"
                            .into(),
                    ))
                }
            }
            #[cfg(target_os = "linux")]
            RawWindowHandle::Xlib(w) => {
                if let RawDisplayHandle::Xlib(d) = display_handle {
                    let display = d.display.ok_or_else(|| {
                        Err(RendererError::new_with_error(
                            RendererErrorKind::UnsupportedWindow,
                            "Xlib display does not contain a required display pointer".into(),
                        ))
                    })?;
                    Ok(Self::Xlib {
                        display,
                        window: w.window as i32,
                    })
                } else {
                    Err(RendererError::new_with_error(
                        RendererErrorKind::UnsupportedWindow,
                        "Display handle must be Xlib variant because window handle is Xlib variant"
                            .into(),
                    ))
                }
            }
            _ => Err(RendererError::new_with_error(
                RendererErrorKind::UnsupportedWindow,
                "Display is not supported on this platform".into(),
            )),
        }
    }
}

pub struct FrameBuffer {
    pub(crate) handle: sys::Cobalt_FrameBuffer,
    _renderer: Arc<RendererInternal>,
}

impl FrameBuffer {
    pub(crate) fn new(
        handle: sys::Cobalt_FrameBuffer,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        FrameBuffer {
            handle,
            _renderer: renderer_internal,
        }
    }

    /// # Safety
    ///
    /// [`Window`] contains raw pointers to system displays and windows. These must be valid pointers
    /// and the resources they point to must be valid for the lifetime of the framebuffer.
    /// This includes during it's deferred deletion at the beginning of the next frame.
    ///
    /// To avoid potential use after free, it's recommended to drop the framebuffer, wait
    /// for it's deletion using [`crate::renderer::Renderer::wait_for_deferred_deletion_complete`],
    /// then delete the window.
    ///
    /// When the window is resized, a call to [`FrameBuffer::notify_window_resized`] should be called.
    pub unsafe fn bind_window(
        &mut self,
        window: Window,
        window_size: &[u32; 2],
        depth_stencil_mode: WindowDepthStencilMode,
        color_space_mode: WindowColorSpaceMode,
        binding_flags: WindowBindingFlags,
    ) -> RendererResult<()> {
        match window {
            #[cfg(target_os = "windows")]
            Window::Win32 { hinstance, hwnd } => unsafe {
                let window_info = sys::Cobalt_WindowInfoWin32 {
                    base: sys::Cobalt_WindowInfoBase {
                        type_: sys::Cobalt_WindowType_Win32,
                        windowSizeInPixels: *window_size,
                    },
                    instanceHandle: hinstance.get() as *mut c_void,
                    windowHandle: hwnd.get() as *mut c_void,
                };
                return_on_failure!(sys::Cobalt_FrameBuffer_BindWindow(
                    self.handle,
                    (&raw const window_info) as *const sys::Cobalt_WindowInfoBase,
                    depth_stencil_mode as sys::Cobalt_WindowDepthStencilMode,
                    color_space_mode as sys::Cobalt_WindowColorSpaceMode,
                    binding_flags.bits() as sys::Cobalt_WindowBindingFlags,
                ));
            },
            #[cfg(target_os = "macos")]
            Window::AppKit { view } => unsafe {
                let window_info = sys::Cobalt_WindowInfoAppKit {
                    base: sys::Cobalt_WindowInfoBase {
                        type_: sys::Cobalt_WindowType_AppKit,
                        windowSizeInPixels: *window_size,
                    },
                    view: view.as_ptr(),
                };
                return_on_failure!(sys::Cobalt_FrameBuffer_BindWindow(
                    self.handle,
                    (&raw const window_info) as *const sys::Cobalt_WindowInfoBase,
                    depth_stencil_mode as sys::Cobalt_WindowDepthStencilMode,
                    color_space_mode as sys::Cobalt_WindowColorSpaceMode,
                    binding_flags.bits() as sys::Cobalt_WindowBindingFlags,
                ));
            },
            #[cfg(target_os = "linux")]
            Window::Wayland { display, surface } => unsafe {
                let window_info = sys::Cobalt_WindowInfoWayland {
                    base: sys::Cobalt_WindowInfoBase {
                        type_: sys::Cobalt_WindowType_Wayland,
                        windowSizeInPixels: *window_size,
                    },
                    display: display.as_ptr(),
                    surface: surface.as_ptr(),
                };
                return_on_failure!(sys::Cobalt_FrameBuffer_BindWindow(
                    self.handle,
                    (&raw const window_info) as *const sys::Cobalt_WindowInfoBase,
                    depth_stencil_mode as sys::Cobalt_WindowDepthStencilMode,
                    color_space_mode as sys::Cobalt_WindowColorSpaceMode,
                    binding_flags.bits() as sys::Cobalt_WindowBindingFlags,
                ));
            },
            #[cfg(target_os = "linux")]
            Window::Xlib { display, window } => unsafe {
                let window_info = sys::Cobalt_WindowInfoXlib {
                    base: sys::Cobalt_WindowInfoBase {
                        type_: sys::Cobalt_WindowType_Xlib,
                        windowSizeInPixels: *window_size,
                    },
                    display: display.as_ptr(),
                    window: window as u64,
                };
                return_on_failure!(sys::Cobalt_FrameBuffer_BindWindow(
                    self.handle,
                    (&raw const window_info) as *const sys::Cobalt_WindowInfoBase,
                    depth_stencil_mode as sys::Cobalt_WindowDepthStencilMode,
                    color_space_mode as sys::Cobalt_WindowColorSpaceMode,
                    binding_flags.bits() as sys::Cobalt_WindowBindingFlags,
                ));
            },
            #[cfg(target_os = "linux")]
            Window::Xcb { connection, window } => unsafe {
                let window_info = sys::Cobalt_WindowInfoXCB {
                    base: sys::Cobalt_WindowInfoBase {
                        type_: sys::Cobalt_WindowType_XCB,
                        windowSizeInPixels: *window_size,
                    },
                    connection: connection.as_ptr(),
                    window: window as u32,
                };
                return_on_failure!(sys::Cobalt_FrameBuffer_BindWindow(
                    self.handle,
                    (&raw const window_info) as *const sys::Cobalt_WindowInfoBase,
                    depth_stencil_mode as sys::Cobalt_WindowDepthStencilMode,
                    color_space_mode as sys::Cobalt_WindowColorSpaceMode,
                    binding_flags.bits() as sys::Cobalt_WindowBindingFlags,
                ));
            },
        }
        Ok(())
    }

    pub fn notify_window_resized(&mut self, window_size: &[u32; 2]) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_FrameBuffer_NotifyWindowResized(
                self.handle,
                window_size
            ))
        }
        Ok(())
    }

    pub fn define_viewport_region(&mut self, start_pos: &[u32; 2], size: &[u32; 2]) {
        unsafe {
            sys::Cobalt_FrameBuffer_DefineViewportRegion(self.handle, start_pos, size);
        }
    }

    pub fn bind_texture(
        &mut self,
        texture: &mut TextureBuffer2D,
        attachment_type: AttachmentType,
        index: usize,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_FrameBuffer_BindTexture(
                self.handle,
                texture.handle,
                attachment_type as sys::Cobalt_AttachmentType,
                index,
            ))
        }
        Ok(())
    }

    pub fn unbind_texture(&mut self, attachment_type: AttachmentType, index: usize) {
        unsafe {
            sys::Cobalt_FrameBuffer_UnbindTexture(
                self.handle,
                attachment_type as sys::Cobalt_AttachmentType,
                index,
            )
        }
    }

    pub fn bind_multi_sampling_resolve_texture(
        &mut self,
        texture: &mut TextureBuffer2D,
        attachment_type: AttachmentType,
        index: usize,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_FrameBuffer_BindMultiSamplingResolveTexture(
                self.handle,
                texture.handle,
                attachment_type as sys::Cobalt_AttachmentType,
                index,
            ))
        }
        Ok(())
    }

    pub fn unbind_sampling_resolve_texture(
        &mut self,
        attachment_type: AttachmentType,
        index: usize,
    ) {
        unsafe {
            sys::Cobalt_FrameBuffer_UnbindMultiSamplingResolveTexture(
                self.handle,
                attachment_type as sys::Cobalt_AttachmentType,
                index,
            )
        }
    }

    pub fn define_scissor_region(&mut self, start_pos: &[u32; 2], size: &[u32; 2]) {
        unsafe {
            sys::Cobalt_FrameBuffer_DefineScissorRegion(self.handle, start_pos, size);
        }
    }

    pub fn remove_scissor_region(&mut self) {
        unsafe {
            sys::Cobalt_FrameBuffer_RemoveScissorRegion(self.handle);
        }
    }

    pub fn add_output_capture_target(
        &mut self,
        target: &mut FrameBufferOutput,
        attachment_type: AttachmentType,
        index: usize,
    ) {
        unsafe {
            sys::Cobalt_FrameBuffer_AddOutputCaptureTarget(
                self.handle,
                target.handle,
                attachment_type as sys::Cobalt_AttachmentType,
                index,
            )
        }
    }

    pub fn remove_output_capture_target(&mut self, target: &mut FrameBufferOutput) {
        unsafe { sys::Cobalt_FrameBuffer_RemoveOutputCaptureTarget(self.handle, target.handle) }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_FrameBuffer_Delete(self.handle);
        }
    }
}

unsafe impl Send for FrameBuffer {}
unsafe impl Sync for FrameBuffer {}
