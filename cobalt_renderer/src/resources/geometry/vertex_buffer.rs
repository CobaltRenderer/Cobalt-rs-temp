// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use std::sync::Arc;

use super::VertexAttribute;
use crate::RendererResult;
use crate::renderer::RendererInternal;
use crate::resources::batching::TransferBatch;
use crate::resources::data::TexelArray;

use cobalt_renderer_sys as sys;

pub struct VertexBuffer {
    pub(crate) handle: sys::Cobalt_VertexBuffer,
    _renderer: Arc<RendererInternal>,
}

impl VertexBuffer {
    pub(crate) fn new(
        handle: sys::Cobalt_VertexBuffer,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        VertexBuffer {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn allocate_memory(&mut self) -> RendererResult<()> {
        unsafe { return_on_failure!(sys::Cobalt_VertexBuffer_AllocateMemory(self.handle)) }
        Ok(())
    }

    pub fn allocate_memory_with_alias(
        &mut self,
        texel_array: &mut TexelArray,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_VertexBuffer_AllocateMemoryWithAlias(
                self.handle,
                texel_array.handle,
            ))
        }
        Ok(())
    }

    pub fn bind_vertex_attribute(&mut self, attribute: &mut VertexAttribute) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_VertexBuffer_BindVertexAttribute(
                self.handle,
                attribute.handle,
            ))
        }
        Ok(())
    }

    pub fn bind_vertex_attribute_manual_layout(
        &mut self,
        attribute: &mut VertexAttribute,
        buffer_offset_in_bytes: usize,
        buffer_stride_in_bytes: usize,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_VertexBuffer_BindVertexAttributeManualLayout(
                self.handle,
                attribute.handle,
                buffer_offset_in_bytes,
                buffer_stride_in_bytes,
            ))
        }
        Ok(())
    }

    pub fn set_raw_initial_data<T: Sized>(&mut self, data: &[T]) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_VertexBuffer_SetRawInitialData(
                self.handle,
                data.as_ptr() as *const u8,
                std::mem::size_of_val(data),
            ))
        }
        Ok(())
    }

    pub fn queue_raw_data_update<T: Sized>(
        &mut self,
        data: &[T],
        buffer_offset_in_bytes: usize,
        transfer_batch: Option<&TransferBatch>,
    ) -> RendererResult<()> {
        let batch = match transfer_batch {
            None => std::ptr::null_mut(),
            Some(b) => b.handle,
        };
        unsafe {
            return_on_failure!(sys::Cobalt_VertexBuffer_QueueRawDataUpdate(
                self.handle,
                data.as_ptr() as *const u8,
                std::mem::size_of_val(data),
                buffer_offset_in_bytes,
                batch,
            ))
        }
        Ok(())
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_VertexBuffer_Delete(self.handle);
        }
    }
}

unsafe impl Send for VertexBuffer {}
unsafe impl Sync for VertexBuffer {}
