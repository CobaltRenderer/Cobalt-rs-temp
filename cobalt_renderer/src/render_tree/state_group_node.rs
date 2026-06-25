// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use std::sync::Arc;

use super::{RenderableNode, StateContainer};
use crate::renderer::RendererInternal;
use crate::resources::frame_buffers::AttachmentType;

use cobalt_renderer_sys as sys;

/// How polygons should be filled when rendered
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolygonFillMode {
    Solid = sys::Cobalt_PolygonFillMode_Solid as i32,
    Wireframe = sys::Cobalt_PolygonFillMode_Wireframe as i32,
}

/// How polygons should be culled when rendered
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolygonCullMode {
    None = sys::Cobalt_PolygonCullMode_None as i32,
    Front = sys::Cobalt_PolygonCullMode_Front as i32,
    Back = sys::Cobalt_PolygonCullMode_Back as i32,
}

/// Defines the winding order which determines if a polygon face forward or backwards
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolygonWindingOrder {
    Clockwise = sys::Cobalt_PolygonWindingOrder_Clockwise as i32,
    CounterClockwise = sys::Cobalt_PolygonWindingOrder_CounterClockwise as i32,
}

/// Comparison functions between two depth value
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DepthComparisonFunction {
    Never = sys::Cobalt_DepthComparisonFunction_Never as i32,
    Equal = sys::Cobalt_DepthComparisonFunction_Equal as i32,
    NotEqual = sys::Cobalt_DepthComparisonFunction_NotEqual as i32,
    Less = sys::Cobalt_DepthComparisonFunction_Less as i32,
    LessOrEqual = sys::Cobalt_DepthComparisonFunction_LessOrEqual as i32,
    Greater = sys::Cobalt_DepthComparisonFunction_Greater as i32,
    GreaterOrEqual = sys::Cobalt_DepthComparisonFunction_GreaterOrEqual as i32,
    Always = sys::Cobalt_DepthComparisonFunction_Always as i32,
}

/// Target face directions for stencil buffer
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StencilTargetFace {
    FrontFace = sys::Cobalt_StencilTargetFace_FrontFace as i32,
    BackFace = sys::Cobalt_StencilTargetFace_BackFace as i32,
    FrontAndBackFace = sys::Cobalt_StencilTargetFace_FrontAndBackFace as i32,
}

/// Comparison function between two stencil values
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StencilComparisonFunction {
    Never = sys::Cobalt_StencilComparisonFunction_Never as i32,
    Equal = sys::Cobalt_StencilComparisonFunction_Equal as i32,
    NotEqual = sys::Cobalt_StencilComparisonFunction_NotEqual as i32,
    Less = sys::Cobalt_StencilComparisonFunction_Less as i32,
    LessOrEqual = sys::Cobalt_StencilComparisonFunction_LessOrEqual as i32,
    Greater = sys::Cobalt_StencilComparisonFunction_Greater as i32,
    GreaterOrEqual = sys::Cobalt_StencilComparisonFunction_GreaterOrEqual as i32,
    Always = sys::Cobalt_StencilComparisonFunction_Always as i32,
}

/// Operation to combine an existing stencil value with a new one
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StencilOperation {
    Keep = sys::Cobalt_StencilOperation_Keep as i32,
    Zero = sys::Cobalt_StencilOperation_Zero as i32,
    Replace = sys::Cobalt_StencilOperation_Replace as i32,
    IncrementAndClamp = sys::Cobalt_StencilOperation_IncrementAndClamp as i32,
    DecrementAndClamp = sys::Cobalt_StencilOperation_DecrementAndClamp as i32,
    IncrementAndWrap = sys::Cobalt_StencilOperation_IncrementAndWrap as i32,
    DecrementAndWrap = sys::Cobalt_StencilOperation_DecrementAndWrap as i32,
    Invert = sys::Cobalt_StencilOperation_Invert as i32,
}

/// Operation to combine an existing destination color value with a new source color
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendOperation {
    Add = sys::Cobalt_BlendOperation_Add as i32,
    Subtract = sys::Cobalt_BlendOperation_Subtract as i32,
    ReverseSubtract = sys::Cobalt_BlendOperation_ReverseSubtract as i32,
    Min = sys::Cobalt_BlendOperation_Min as i32,
    Max = sys::Cobalt_BlendOperation_Max as i32,
}

/// Factors to use when combining an existing destination color value with a new source color
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendFactor {
    Zero = sys::Cobalt_BlendFactor_Zero as i32,
    One = sys::Cobalt_BlendFactor_One as i32,
    SourceColor = sys::Cobalt_BlendFactor_SourceColor as i32,
    OneMinusSourceColor = sys::Cobalt_BlendFactor_OneMinusSourceColor as i32,
    DestinationColor = sys::Cobalt_BlendFactor_DestinationColor as i32,
    OneMinusDestinationColor = sys::Cobalt_BlendFactor_OneMinusDestinationColor as i32,
    SourceAlpha = sys::Cobalt_BlendFactor_SourceAlpha as i32,
    OneMinusSourceAlpha = sys::Cobalt_BlendFactor_OneMinusSourceAlpha as i32,
    DestinationAlpha = sys::Cobalt_BlendFactor_DestinationAlpha as i32,
    OneMinusDestinationAlpha = sys::Cobalt_BlendFactor_OneMinusDestinationAlpha as i32,
}

/// Describes how to blend a new color with an existing color in the frame buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlendDescriptor {
    pub operation_rgb: BlendOperation,
    pub factor_source_rgb: BlendFactor,
    pub factor_destination_rgb: BlendFactor,
    pub operation_a: BlendOperation,
    pub factor_source_a: BlendFactor,
    pub factor_destination_a: BlendFactor,
}

/// Defines graphics state and how content is drawn
pub struct StateGroupNode {
    pub(crate) handle: sys::Cobalt_StateGroupNode,
    _renderer: Arc<RendererInternal>,
}

impl StateGroupNode {
    pub(crate) fn new(
        handle: sys::Cobalt_StateGroupNode,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        StateGroupNode {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn add_child_node(&self, child_node: &RenderableNode) {
        unsafe {
            sys::Cobalt_StateGroupNode_AddChildNode(self.handle, child_node.handle);
        }
    }

    pub fn add_child_nodes(&self, child_nodes: &[&RenderableNode]) {
        unsafe {
            let nodes: Vec<sys::Cobalt_RenderableNode> =
                child_nodes.iter().map(|n| n.handle).collect();
            sys::Cobalt_StateGroupNode_AddChildNodes(self.handle, nodes.as_ptr(), nodes.len());
        }
    }

    pub fn remove_child_node(&self, child_node: &RenderableNode) {
        unsafe {
            sys::Cobalt_StateGroupNode_RemoveChildNode(self.handle, child_node.handle);
        }
    }

    pub fn remove_child_nodes(&self, child_nodes: &[&RenderableNode]) {
        unsafe {
            let nodes: Vec<sys::Cobalt_RenderableNode> =
                child_nodes.iter().map(|n| n.handle).collect();
            sys::Cobalt_StateGroupNode_RemoveChildNodes(self.handle, nodes.as_ptr(), nodes.len());
        }
    }

    pub fn remove_all_child_nodes(&self) {
        unsafe {
            sys::Cobalt_StateGroupNode_RemoveAllChildNodes(self.handle);
        }
    }

    pub fn set_child_nodes(&self, child_nodes: &[&RenderableNode]) {
        unsafe {
            let nodes: Vec<sys::Cobalt_RenderableNode> =
                child_nodes.iter().map(|n| n.handle).collect();
            sys::Cobalt_StateGroupNode_SetChildNodes(self.handle, nodes.as_ptr(), nodes.len());
        }
    }

    pub fn set_compute_task(&self, thread_group_counts: impl AsRef<[u32; 3]>) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetComputeTask(self.handle, thread_group_counts.as_ref());
        }
    }

    pub fn remove_compute_task(&self) {
        unsafe {
            sys::Cobalt_StateGroupNode_RemoveComputeTask(self.handle);
        }
    }

    pub fn set_depth_test_enabled(&mut self, enabled: bool) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetDepthTestEnabled(
                self.handle,
                if enabled { 1 } else { 0 },
            );
        }
    }

    pub fn set_depth_write_enabled(&mut self, enabled: bool) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetDepthWriteEnabled(
                self.handle,
                if enabled { 1 } else { 0 },
            );
        }
    }

    pub fn set_depth_comparison_function(&mut self, comparison_test: DepthComparisonFunction) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetDepthComparisonFunction(
                self.handle,
                comparison_test as sys::Cobalt_DepthComparisonFunction,
            );
        }
    }

    pub fn set_depth_bias(&mut self, constant_factor: f32, slope_factor: f32, clamp: Option<f32>) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetDepthBias(
                self.handle,
                constant_factor,
                slope_factor,
                clamp.unwrap_or(0.0),
            );
        }
    }

    pub fn clear_depth_bias(&mut self) {
        unsafe {
            sys::Cobalt_StateGroupNode_ClearDepthBias(self.handle);
        }
    }

    pub fn set_stencil_test_enabled(
        &mut self,
        enabled: bool,
        compare_mask: Option<u32>,
        write_mask: Option<u32>,
    ) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetStencilTestEnabled(
                self.handle,
                if enabled { 1 } else { 0 },
                compare_mask.unwrap_or(u32::MAX),
                write_mask.unwrap_or(u32::MAX),
            );
        }
    }

    pub fn set_stencil_operation(
        &mut self,
        target_face: StencilTargetFace,
        comparison_test: StencilComparisonFunction,
        pass_operation: StencilOperation,
        fail_operation: StencilOperation,
        depth_fail_operation: StencilOperation,
    ) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetStencilOperation(
                self.handle,
                target_face as sys::Cobalt_StencilTargetFace,
                comparison_test as sys::Cobalt_StencilComparisonFunction,
                pass_operation as sys::Cobalt_StencilOperation,
                fail_operation as sys::Cobalt_StencilOperation,
                depth_fail_operation as sys::Cobalt_StencilOperation,
            );
        }
    }

    pub fn set_stencil_reference_value(&mut self, reference_value: u32) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetStencilReferenceValue(self.handle, reference_value);
        }
    }

    pub fn set_polygon_fill_mode(&mut self, fill_mode: PolygonFillMode) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetPolygonFillMode(
                self.handle,
                fill_mode as sys::Cobalt_PolygonFillMode,
            );
        }
    }

    pub fn set_polygon_cull_mode(&mut self, cull_mode: PolygonCullMode) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetPolygonCullMode(
                self.handle,
                cull_mode as sys::Cobalt_PolygonCullMode,
            );
        }
    }

    pub fn set_polygon_winding_order(&mut self, winding_order: PolygonWindingOrder) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetPolygonWindingOrder(
                self.handle,
                winding_order as sys::Cobalt_PolygonWindingOrder,
            );
        }
    }

    pub fn set_blend_enabled(&mut self, enabled: bool) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetBlendEnabled(self.handle, if enabled { 1 } else { 0 });
        }
    }

    pub fn set_shared_blend_mode(&mut self, blend_descriptor: &BlendDescriptor) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetSharedBlendMode(
                self.handle,
                blend_descriptor.operation_rgb as sys::Cobalt_BlendOperation,
                blend_descriptor.factor_source_rgb as sys::Cobalt_BlendFactor,
                blend_descriptor.factor_destination_rgb as sys::Cobalt_BlendFactor,
                blend_descriptor.operation_a as sys::Cobalt_BlendOperation,
                blend_descriptor.factor_source_a as sys::Cobalt_BlendFactor,
                blend_descriptor.factor_destination_a as sys::Cobalt_BlendFactor,
            );
        }
    }

    pub fn set_blend_mode(
        &mut self,
        attachment_type: AttachmentType,
        index: usize,
        blend_descriptor: &BlendDescriptor,
    ) {
        unsafe {
            sys::Cobalt_StateGroupNode_SetBlendMode(
                self.handle,
                attachment_type as sys::Cobalt_AttachmentType,
                index,
                blend_descriptor.operation_rgb as sys::Cobalt_BlendOperation,
                blend_descriptor.factor_source_rgb as sys::Cobalt_BlendFactor,
                blend_descriptor.factor_destination_rgb as sys::Cobalt_BlendFactor,
                blend_descriptor.operation_a as sys::Cobalt_BlendOperation,
                blend_descriptor.factor_source_a as sys::Cobalt_BlendFactor,
                blend_descriptor.factor_destination_a as sys::Cobalt_BlendFactor,
            );
        }
    }

    pub fn set_debug_name(&mut self, name: impl AsRef<str>) {
        let bytes = name.as_ref().as_bytes();
        unsafe {
            sys::Cobalt_StateGroupNode_SetDebugName(
                self.handle,
                bytes.as_ptr() as *const std::ffi::c_char,
                bytes.len(),
            );
        }
    }
}

impl StateContainer for StateGroupNode {
    fn node_handle(&mut self) -> sys::Cobalt_StateContainer {
        self.handle as sys::Cobalt_StateContainer
    }
}

impl Drop for StateGroupNode {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_StateGroupNode_Delete(self.handle);
        }
    }
}

unsafe impl Send for StateGroupNode {}
unsafe impl Sync for StateGroupNode {}
