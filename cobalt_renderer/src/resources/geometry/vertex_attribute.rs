// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use std::sync::Arc;

use super::{DataPersistenceFlags, PerformanceHint};
use crate::renderer::{Renderer, RendererInternal};
use crate::resources::batching::TransferBatch;
use crate::{RendererError, RendererResult};

use cobalt_renderer_sys as sys;

/// Data type of a vertex attribute
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexAttributeType {
    Int8 = sys::Cobalt_VertexAttributeType_Int8 as i32,
    Int16 = sys::Cobalt_VertexAttributeType_Int16 as i32,
    Int32 = sys::Cobalt_VertexAttributeType_Int32 as i32,
    UInt8 = sys::Cobalt_VertexAttributeType_UInt8 as i32,
    UInt16 = sys::Cobalt_VertexAttributeType_UInt16 as i32,
    UInt32 = sys::Cobalt_VertexAttributeType_UInt32 as i32,
    Norm8 = sys::Cobalt_VertexAttributeType_Norm8 as i32,
    Norm16 = sys::Cobalt_VertexAttributeType_Norm16 as i32,
    UNorm8 = sys::Cobalt_VertexAttributeType_UNorm8 as i32,
    UNorm16 = sys::Cobalt_VertexAttributeType_UNorm16 as i32,
    Float16 = sys::Cobalt_VertexAttributeType_Float16 as i32,
    Float32 = sys::Cobalt_VertexAttributeType_Float32 as i32,
    A2B10G10R10UNorm = sys::Cobalt_VertexAttributeType_A2B10G10R10UNorm as i32,
}

/// Vertex attribute input for a shader program, backed by a vertex buffer
pub struct VertexAttribute {
    pub(crate) handle: sys::Cobalt_VertexAttribute,
    _renderer: Arc<RendererInternal>,
    element_size: usize,
}

impl VertexAttribute {
    pub(crate) fn new(
        renderer: &Renderer,
        data_type: VertexAttributeType,
        element_count: usize,
        vertex_count: usize,
        performance_hint_cpu: PerformanceHint,
        performance_hint_gpu: PerformanceHint,
        data_persistence_flags: DataPersistenceFlags,
    ) -> VertexAttribute {
        assert!(
            element_count > 0 && element_count <= 4,
            "VertexAttribute must have element count from 1 to 4, cannot be {element_count}"
        );

        let data_size = match data_type {
            VertexAttributeType::Int8 => 1,
            VertexAttributeType::Int16 => 2,
            VertexAttributeType::Int32 => 4,
            VertexAttributeType::UInt8 => 1,
            VertexAttributeType::UInt16 => 2,
            VertexAttributeType::UInt32 => 4,
            VertexAttributeType::Norm8 => 1,
            VertexAttributeType::Norm16 => 2,
            VertexAttributeType::UNorm8 => 1,
            VertexAttributeType::UNorm16 => 2,
            VertexAttributeType::Float16 => 2,
            VertexAttributeType::Float32 => 4,
            VertexAttributeType::A2B10G10R10UNorm => 4,
        };

        let mut vertex_attribute = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateVertexAttribute(
                renderer.handle(),
                &mut vertex_attribute,
                data_type as sys::Cobalt_VertexAttributeType,
                element_count,
                vertex_count,
                performance_hint_cpu.bits() as sys::Cobalt_VertexPerformanceHint,
                performance_hint_gpu.bits() as sys::Cobalt_VertexPerformanceHint,
                data_persistence_flags.bits() as sys::Cobalt_VertexDataPersistenceFlags,
            );
        }

        VertexAttribute {
            _renderer: renderer.internal_clone(),
            handle: vertex_attribute,
            element_size: data_size * element_count,
        }
    }

    pub fn is_bound_to_buffer(&self) -> bool {
        unsafe { sys::Cobalt_VertexAttribute_IsBoundToBuffer(self.handle) != 0 }
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
            return_on_failure!(sys::Cobalt_VertexAttribute_SetInitialData(
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
            return_on_failure!(sys::Cobalt_VertexAttribute_QueueDataUpdate(
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

impl Drop for VertexAttribute {
    fn drop(&mut self) {
        unsafe { sys::Cobalt_VertexAttribute_Delete(self.handle) }
    }
}

unsafe impl Send for VertexAttribute {}
unsafe impl Sync for VertexAttribute {}
