// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use std::sync::Arc;

use super::VertexAttribute;
use crate::RendererResult;
use crate::renderer::RendererInternal;
use crate::resources::batching::TransferBatch;
use crate::resources::data::TexelArray;

use cobalt_renderer_sys as sys;

pub struct UnallocatedVertexBuffer<'a> {
    vertex_buffer: VertexBuffer,
    _initial_data: std::marker::PhantomData<&'a i32>,
}

impl<'a> UnallocatedVertexBuffer<'a> {
    pub(crate) fn new(
        handle: sys::Cobalt_VertexBuffer,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        UnallocatedVertexBuffer {
            vertex_buffer: VertexBuffer::new(handle, renderer_internal),
            _initial_data: std::marker::PhantomData,
        }
    }

    pub fn bind_attribute(&mut self, attribute: &mut VertexAttribute) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_VertexBuffer_BindVertexAttribute(
                self.vertex_buffer.handle,
                attribute.handle,
            ))
        }
        Ok(())
    }

    pub fn bind_attribute_with_initial_data<S: Sized>(
        &mut self,
        attribute: &mut VertexAttribute,
        data: &'a [S],
        entry_stride_in_bytes: Option<usize>,
    ) -> RendererResult<()> {
        self.bind_attribute(attribute)?;
        unsafe {
            return_on_failure!(sys::Cobalt_VertexAttribute_SetInitialData(
                attribute.handle,
                data.as_ptr() as *const u8,
                core::mem::size_of_val(data) / attribute.element_size,
                entry_stride_in_bytes.unwrap_or(attribute.element_size),
            ))
        }
        Ok(())
    }

    pub fn bind_attribute_manual_layout(
        &mut self,
        attribute: &mut VertexAttribute,
        buffer_offset_in_bytes: usize,
        buffer_stride_in_bytes: usize,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_VertexBuffer_BindVertexAttributeManualLayout(
                self.vertex_buffer.handle,
                attribute.handle,
                buffer_offset_in_bytes,
                buffer_stride_in_bytes,
            ))
        }
        Ok(())
    }

    pub fn bind_attribute_manual_layout_with_initial_data<S: Sized>(
        &mut self,
        attribute: &mut VertexAttribute,
        buffer_offset_in_bytes: usize,
        buffer_stride_in_bytes: usize,
        data: &'a [S],
        entry_stride_in_bytes: Option<usize>,
    ) -> RendererResult<()> {
        self.bind_attribute_manual_layout(
            attribute,
            buffer_offset_in_bytes,
            buffer_stride_in_bytes,
        )?;
        unsafe {
            return_on_failure!(sys::Cobalt_VertexAttribute_SetInitialData(
                attribute.handle,
                data.as_ptr() as *const u8,
                core::mem::size_of_val(data) / attribute.element_size,
                entry_stride_in_bytes.unwrap_or(attribute.element_size),
            ))
        }
        Ok(())
    }

    pub fn set_raw_initial_data<S: Sized>(&mut self, data: &'a [S]) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_VertexBuffer_SetRawInitialData(
                self.vertex_buffer.handle,
                data.as_ptr() as *const u8,
                std::mem::size_of_val(data),
            ))
        }
        Ok(())
    }

    pub fn allocate_memory(self) -> RendererResult<VertexBuffer> {
        unsafe {
            return_on_failure!(sys::Cobalt_VertexBuffer_AllocateMemory(
                self.vertex_buffer.handle
            ))
        }
        Ok(self.vertex_buffer)
    }

    pub fn allocate_memory_with_alias(
        self,
        texel_array: &mut TexelArray,
    ) -> RendererResult<VertexBuffer> {
        unsafe {
            return_on_failure!(sys::Cobalt_VertexBuffer_AllocateMemoryWithAlias(
                self.vertex_buffer.handle,
                texel_array.handle,
            ))
        }
        Ok(self.vertex_buffer)
    }
}

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
