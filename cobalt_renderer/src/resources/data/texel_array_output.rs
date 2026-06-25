// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use num_enum::TryFromPrimitive;
use std::sync::Arc;

use super::{SourceDataFormat, SourceImageFormat};
use crate::renderer::RendererInternal;
use crate::{RendererError, RendererResult};

use cobalt_renderer_sys as sys;

/// Capture are read a texel array to CPU memory
pub struct TexelArrayOutput {
    pub(crate) handle: sys::Cobalt_TexelArrayOutput,
    _renderer: Arc<RendererInternal>,
}

impl TexelArrayOutput {
    pub(crate) fn new(
        handle: sys::Cobalt_TexelArrayOutput,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        TexelArrayOutput {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn set_detach_after_capture(&mut self, should_detach: bool) {
        unsafe {
            sys::Cobalt_TexelArrayOutput_SetDetachAfterCapture(
                self.handle,
                if should_detach { 1 } else { 0 },
            );
        }
    }

    pub fn set_array_capture_region(&mut self, capture_entry_count: usize, buffer_offset: usize) {
        unsafe {
            sys::Cobalt_TexelArrayOutput_SetArrayCaptureRegion(
                self.handle,
                capture_entry_count,
                buffer_offset,
            )
        }
    }

    pub fn has_captured_output(&self) -> bool {
        unsafe { sys::Cobalt_TexelArrayOutput_HasCapturedOutput(self.handle) != 0 }
    }

    pub fn clear_captured_output(&mut self) {
        unsafe { sys::Cobalt_TexelArrayOutput_ClearCapturedOutput(self.handle) }
    }

    pub fn entry_count(&self) -> usize {
        unsafe { sys::Cobalt_TexelArrayOutput_GetEntryCount(self.handle) }
    }

    pub fn optimal_image_format(&self) -> SourceImageFormat {
        let value = unsafe { sys::Cobalt_TexelArrayOutput_GetOptimalImageFormat(self.handle) };
        SourceImageFormat::try_from_primitive(value as i32).unwrap()
    }

    pub fn optimal_data_format(&self) -> SourceDataFormat {
        let value = unsafe { sys::Cobalt_TexelArrayOutput_GetOptimalDataFormat(self.handle) };
        SourceDataFormat::try_from_primitive(value as i32).unwrap()
    }

    pub fn read_buffer_data<S: Sized>(
        &self,
        buffer: &mut [S],
        image_format: SourceImageFormat,
        data_format: SourceDataFormat,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_TexelArrayOutput_ReadBufferData(
                self.handle,
                buffer.as_ptr() as *mut std::ffi::c_void,
                core::mem::size_of_val(buffer),
                image_format as sys::Cobalt_SourceImageFormat,
                data_format as sys::Cobalt_SourceDataFormat,
            ))
        }
        Ok(())
    }
}

impl Drop for TexelArrayOutput {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_TexelArrayOutput_Delete(self.handle);
        }
    }
}

unsafe impl Send for TexelArrayOutput {}
unsafe impl Sync for TexelArrayOutput {}
