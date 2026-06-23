// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License
use num_enum::TryFromPrimitive;
use std::sync::Arc;

use crate::renderer::RendererInternal;
use crate::resources::textures::{SourceDataFormat, SourceImageFormat};
use crate::{RendererError, RendererResult};

use cobalt_renderer_sys as sys;

// Capture and read a frame buffer to CPU memory
pub struct FrameBufferOutput {
    pub(crate) handle: sys::Cobalt_FrameBufferOutput,
    _renderer: Arc<RendererInternal>,
}

impl FrameBufferOutput {
    pub(crate) fn new(
        handle: sys::Cobalt_FrameBufferOutput,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        FrameBufferOutput {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn set_detach_after_capture(&mut self, should_detach: bool) {
        unsafe {
            sys::Cobalt_FrameBufferOutput_SetDetachAfterCapture(
                self.handle,
                if should_detach { 1 } else { 0 },
            )
        }
    }

    pub fn set_frame_buffer_capture_region(
        &mut self,
        image_offset_in_pixels: &[u32; 2],
        image_region_in_pixels: &[u32; 2],
    ) {
        unsafe {
            sys::Cobalt_FrameBufferOutput_SetFrameBufferCaptureRegion(
                self.handle,
                image_offset_in_pixels,
                image_region_in_pixels,
            )
        }
    }

    pub fn has_captured_output(&self) -> bool {
        unsafe { sys::Cobalt_FrameBufferOutput_HasCapturedOutput(self.handle) != 0 }
    }

    pub fn clear_captured_output(&mut self) {
        unsafe { sys::Cobalt_FrameBufferOutput_ClearCapturedOutput(self.handle) }
    }

    pub fn image_dimensions(&self) -> [u32; 2] {
        unsafe {
            let mut dimensions: [u32; 2] = [0; 2];
            sys::Cobalt_FrameBufferOutput_GetImageDimensions(self.handle, &mut dimensions);
            dimensions
        }
    }

    pub fn cropped_image_dimensions(
        &self,
        image_offset_in_pixels: &[u32; 2],
        image_region_in_pixels: &[u32; 2],
    ) -> [u32; 2] {
        unsafe {
            let mut dimensions: [u32; 2] = [0; 2];
            sys::Cobalt_FrameBufferOutput_GetCroppedImageDimensions(
                self.handle,
                image_offset_in_pixels,
                image_region_in_pixels,
                &mut dimensions,
            );
            dimensions
        }
    }

    pub fn optimal_image_format(&self) -> SourceImageFormat {
        let value = unsafe { sys::Cobalt_FrameBufferOutput_GetOptimalImageFormat(self.handle) };
        SourceImageFormat::try_from_primitive(value as i32).unwrap()
    }

    pub fn optimal_data_format(&self) -> SourceDataFormat {
        let value = unsafe { sys::Cobalt_FrameBufferOutput_GetOptimalDataFormat(self.handle) };
        SourceDataFormat::try_from_primitive(value as i32).unwrap()
    }

    pub fn read_buffer_data<S: Sized>(
        &mut self,
        buffer: &mut [S],
        image_format: SourceImageFormat,
        data_format: SourceDataFormat,
        image_offset_in_pixels: &[u32; 2],
        image_region_in_pixels: &[u32; 2],
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_FrameBufferOutput_ReadBufferData(
                self.handle,
                buffer.as_mut_ptr() as *mut std::ffi::c_void,
                core::mem::size_of_val(buffer),
                image_format as sys::Cobalt_SourceImageFormat,
                data_format as sys::Cobalt_SourceDataFormat,
                image_offset_in_pixels,
                image_region_in_pixels,
            ))
        }
        Ok(())
    }
}

impl Drop for FrameBufferOutput {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_FrameBufferOutput_Delete(self.handle);
        }
    }
}

unsafe impl Send for FrameBufferOutput {}
unsafe impl Sync for FrameBufferOutput {}
