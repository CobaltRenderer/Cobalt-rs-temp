// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use std::sync::Arc;

use super::{DataPersistenceFlags, PerformanceHint};
use crate::renderer::{Renderer, RendererInternal};
use crate::resources::batching::TransferBatch;
use crate::{RendererError, RendererResult};

use cobalt_renderer_sys as sys;

// Data type for a index attribute
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexAttributeType {
    UInt16 = sys::Cobalt_IndexAttributeType_UInt16 as i32,
    UInt32 = sys::Cobalt_IndexAttributeType_UInt32 as i32,
}

// Indices for input assembly backed by an index buffer
pub struct IndexAttribute {
    pub(crate) handle: sys::Cobalt_IndexAttribute,
    _renderer: Arc<RendererInternal>,
    element_size: usize,
}

impl IndexAttribute {
    pub(crate) fn new(
        renderer: &Renderer,
        data_type: IndexAttributeType,
        index_count: usize,
        performance_hint_cpu: PerformanceHint,
        performance_hint_gpu: PerformanceHint,
        data_persistence_flags: DataPersistenceFlags,
    ) -> IndexAttribute {
        let data_size = match data_type {
            IndexAttributeType::UInt16 => 2,
            IndexAttributeType::UInt32 => 4,
        };

        let mut index_attribute = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateIndexAttribute(
                renderer.handle(),
                &mut index_attribute,
                data_type as sys::Cobalt_IndexAttributeType,
                index_count,
                performance_hint_cpu.bits() as sys::Cobalt_IndexPerformanceHint,
                performance_hint_gpu.bits() as sys::Cobalt_IndexPerformanceHint,
                data_persistence_flags.bits() as sys::Cobalt_IndexDataPersistenceFlags,
            );
        }

        IndexAttribute {
            _renderer: renderer.internal_clone(),
            handle: index_attribute,
            element_size: data_size,
        }
    }

    pub fn is_bound_to_buffer(&self) -> bool {
        unsafe { sys::Cobalt_IndexAttribute_IsBoundToBuffer(self.handle) != 0 }
    }

    pub fn set_initial_data<S: Sized>(
        &mut self,
        data: &[S],
        entry_stride_in_bytes: Option<usize>,
    ) -> RendererResult<()> {
        let element_size: usize = core::mem::size_of::<S>();

        assert!(
            self.element_size == element_size,
            "Input element size ({} bytes) must have same size as attribute element size ({} bytes)",
            element_size,
            self.element_size,
        );

        unsafe {
            return_on_failure!(sys::Cobalt_IndexAttribute_SetInitialData(
                self.handle,
                data.as_ptr() as *const u8,
                data.len(),
                entry_stride_in_bytes.unwrap_or(element_size),
            ))
        }
        Ok(())
    }

    pub fn queue_data_update<S: Sized>(
        &mut self,
        data: &[S],
        initial_vertex_no: usize,
        entry_stride_in_bytes: Option<usize>,
        transfer_batch: Option<&TransferBatch>,
    ) -> RendererResult<()> {
        let element_size: usize = core::mem::size_of::<S>();
        assert!(
            self.element_size == element_size,
            "Input element size ({} bytes) must have same size as attribute element size ({} bytes)",
            element_size,
            self.element_size,
        );
        let transfer_batch = match transfer_batch {
            None => std::ptr::null_mut(),
            Some(b) => b.handle,
        };

        unsafe {
            return_on_failure!(sys::Cobalt_IndexAttribute_QueueDataUpdate(
                self.handle,
                data.as_ptr() as *const u8,
                data.len(),
                initial_vertex_no,
                entry_stride_in_bytes.unwrap_or(element_size),
                transfer_batch,
            ))
        }
        Ok(())
    }
}

impl Drop for IndexAttribute {
    fn drop(&mut self) {
        unsafe { sys::Cobalt_IndexAttribute_Delete(self.handle) }
    }
}

unsafe impl Send for IndexAttribute {}
unsafe impl Sync for IndexAttribute {}
