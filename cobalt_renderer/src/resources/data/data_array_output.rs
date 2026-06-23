// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License
use std::sync::Arc;

use crate::renderer::RendererInternal;
use crate::{RendererError, RendererResult};

use cobalt_renderer_sys as sys;

/// Capture are read a data array to CPU memory
pub struct DataArrayOutput {
    pub(crate) handle: sys::Cobalt_DataArrayOutput,
    _renderer: Arc<RendererInternal>,
}

impl DataArrayOutput {
    pub(crate) fn new(
        handle: sys::Cobalt_DataArrayOutput,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        DataArrayOutput {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn set_detach_after_capture(&mut self, should_detach: bool) {
        unsafe {
            sys::Cobalt_DataArrayOutput_SetDetachAfterCapture(
                self.handle,
                if should_detach { 1 } else { 0 },
            )
        }
    }

    pub fn set_array_capture_region(
        &mut self,
        capture_entry_count: usize,
        buffer_offset: usize,
        capture_counter_value: bool,
    ) {
        unsafe {
            sys::Cobalt_DataArrayOutput_SetArrayCaptureRegion(
                self.handle,
                capture_entry_count,
                buffer_offset,
                if capture_counter_value { 1 } else { 0 },
            )
        }
    }

    pub fn has_captured_output(&self) -> bool {
        unsafe { sys::Cobalt_DataArrayOutput_HasCapturedOutput(self.handle) != 0 }
    }

    pub fn has_captured_counter_value(&self) -> bool {
        unsafe { sys::Cobalt_DataArrayOutput_HasCapturedCounterValue(self.handle) != 0 }
    }

    pub fn clear_captured_output(&mut self) {
        unsafe { sys::Cobalt_DataArrayOutput_ClearCapturedOutput(self.handle) }
    }

    pub fn entry_count(&self) -> usize {
        unsafe { sys::Cobalt_DataArrayOutput_GetEntryCount(self.handle) }
    }

    pub fn entry_size_in_bytes(&self) -> usize {
        unsafe { sys::Cobalt_DataArrayOutput_GetEntrySizeInBytes(self.handle) }
    }

    pub fn read_buffer_data<S: Sized>(&self, buffer: &mut [S]) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_DataArrayOutput_ReadBufferData(
                self.handle,
                buffer.as_mut_ptr() as *mut std::ffi::c_void,
                core::mem::size_of_val(buffer),
            ))
        }
        Ok(())
    }

    pub fn read_counter_value(&self) -> RendererResult<u32> {
        let mut value: u32 = 0;
        unsafe {
            return_on_failure!(sys::Cobalt_DataArrayOutput_ReadCounterValue(
                self.handle,
                &mut value
            ))
        }
        Ok(value)
    }
}

impl Drop for DataArrayOutput {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_DataArrayOutput_Delete(self.handle);
        }
    }
}

unsafe impl Send for DataArrayOutput {}
unsafe impl Sync for DataArrayOutput {}
