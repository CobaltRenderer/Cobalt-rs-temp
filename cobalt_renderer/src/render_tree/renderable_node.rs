// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use std::sync::Arc;

use super::StateContainer;
use crate::RendererResult;
use crate::resources::data::DataArray;
use crate::resources::geometry::{IndexAttribute, VertexAttribute};
use crate::{renderer::RendererInternal, resources::VertexAttributeId};

use cobalt_renderer_sys as sys;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveMode {
    Points = sys::Cobalt_PrimitiveMode_Points as i32,
    Lines = sys::Cobalt_PrimitiveMode_Lines as i32,
    Triangles = sys::Cobalt_PrimitiveMode_Triangles as i32,
    LineStrip = sys::Cobalt_PrimitiveMode_LineStrip as i32,
    TriangleStrip = sys::Cobalt_PrimitiveMode_TriangleStrip as i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndirectDrawParams {
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedIndirectDrawParams {
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: u32,
    first_instance: u32,
}

pub struct RenderableNode {
    pub(crate) handle: sys::Cobalt_RenderableNode,
    _renderer: Arc<RendererInternal>,
}

impl RenderableNode {
    pub(crate) fn new(
        handle: sys::Cobalt_RenderableNode,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        RenderableNode {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn bind_vertex_attribute(
        &mut self,
        attribute: &mut VertexAttribute,
        shader_attribute_id: VertexAttributeId,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_RenderableNode_BindVertexAttribute(
                self.handle,
                attribute.handle,
                shader_attribute_id.0,
            ))
        }
        Ok(())
    }

    pub fn bind_vertex_instance_attribute(
        &mut self,
        attribute: &mut VertexAttribute,
        shader_attribute_id: VertexAttributeId,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_RenderableNode_BindVertexInstanceAttribute(
                self.handle,
                attribute.handle,
                shader_attribute_id.0,
            ))
        }
        Ok(())
    }

    pub fn bind_index_attribute(&mut self, attribute: &mut IndexAttribute) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_RenderableNode_BindIndexAttribute(
                self.handle,
                attribute.handle,
            ));
        }
        Ok(())
    }

    pub fn set_primitive_mode(
        &mut self,
        primitive_mode: PrimitiveMode,
        primitive_restart_enabled: bool,
        adjacency_enabled: bool,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_RenderableNode_SetPrimitiveMode(
                self.handle,
                primitive_mode as sys::Cobalt_PrimitiveMode,
                if primitive_restart_enabled { 1 } else { 0 },
                if adjacency_enabled { 1 } else { 0 },
            ))
        }
        Ok(())
    }

    pub fn set_vertex_count(
        &mut self,
        vertex_count: usize,
        vertex_buffer_offset: usize,
        index_buffer_offset: usize,
        index_value_offset: isize,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_RenderableNode_SetVertexCount(
                self.handle,
                vertex_count,
                vertex_buffer_offset,
                index_buffer_offset,
                index_value_offset,
            ))
        }
        Ok(())
    }

    pub fn set_instance_mode(
        &mut self,
        instance_count: u32,
        instance_offset: u32,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_RenderableNode_SetInstanceMode(
                self.handle,
                instance_count,
                instance_offset,
            ))
        }
        Ok(())
    }

    pub fn set_indirect_draw(
        &mut self,
        draw_count: usize,
        source_data_array: &mut DataArray,
        array_offset_in_bytes: usize,
        array_stride_in_bytes: usize,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_RenderableNode_SetIndirectDraw(
                self.handle,
                draw_count,
                source_data_array.handle,
                array_offset_in_bytes,
                array_stride_in_bytes,
            ))
        }
        Ok(())
    }

    pub fn set_indirect_draw_counter(
        &mut self,
        max_draw_count: usize,
        draw_count_source_data_array: &mut DataArray,
        source_data_array: &mut DataArray,
        array_offset_in_bytes: usize,
        array_stride_in_bytes: usize,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_RenderableNode_SetIndirectDrawCounter(
                self.handle,
                max_draw_count,
                draw_count_source_data_array.handle,
                source_data_array.handle,
                array_offset_in_bytes,
                array_stride_in_bytes,
            ))
        }
        Ok(())
    }

    pub fn set_debug_name(&mut self, name: impl AsRef<str>) {
        let bytes = name.as_ref().as_bytes();
        unsafe {
            sys::Cobalt_RenderableNode_SetDebugName(
                self.handle,
                bytes.as_ptr() as *const std::ffi::c_char,
                bytes.len(),
            );
        }
    }
}

impl StateContainer for RenderableNode {
    fn node_handle(&mut self) -> sys::Cobalt_StateContainer {
        self.handle as sys::Cobalt_StateContainer
    }
}

impl Drop for RenderableNode {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_RenderableNode_Delete(self.handle);
        }
    }
}

unsafe impl Send for RenderableNode {}
unsafe impl Sync for RenderableNode {}
