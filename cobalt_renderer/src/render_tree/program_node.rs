// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License
use std::sync::Arc;

use super::StateGroupNode;
use crate::renderer::RendererInternal;
use crate::resources::StateValueId;
use crate::resources::programs::ShaderProgram;
use crate::{RendererError, RendererResult};

use cobalt_renderer_sys as sys;

// This is a workaround for Rust not having generic specialization
// which would allow us to have different functions under the same name
// that would take different input types and have different implementations.
// Instead we have a trait which we only implement on some types
// which then have specializations. Then we can have one generic
// function which takes this trait and calls the specialized function
// on the type. Not ideal but functional

/// A type which can be used to set a shader constant state value, scalar or vector
pub trait ConstantStateValue {
    #[doc(hidden)]
    fn set_constant_state_value(
        &self,
        program_node: &mut ProgramNode,
        state_id: StateValueId,
        array_indices: &[usize],
    );
}

/// A type which can be used to set a shader constant state value, only matrix
pub trait ConstantStateValueMatrix {
    #[doc(hidden)]
    fn set_constant_state_value_matrix(
        &self,
        program_node: &mut ProgramNode,
        state_id: StateValueId,
        array_indices: &[usize],
    );
}

impl ConstantStateValue for bool {
    fn set_constant_state_value(
        &self,
        program_node: &mut ProgramNode,
        state_id: StateValueId,
        array_indices: &[usize],
    ) {
        unsafe {
            sys::Cobalt_ProgramNode_SetConstantStateValueBool(
                program_node.handle,
                state_id.0,
                if *self { 1 } else { 0 },
                array_indices.as_ptr(),
                array_indices.len(),
            );
        }
    }
}

// We use macros to reduce boilerplate, they just take the type
// and function to call and fill in the implementation for it

macro_rules! declare_set_state_value {
    ( $type:ty, $func:path ) => {
        impl ConstantStateValue for $type {
            fn set_constant_state_value(
                &self,
                program_node: &mut ProgramNode,
                state_id: StateValueId,
                array_indices: &[usize],
            ) {
                unsafe {
                    $func(
                        program_node.handle,
                        state_id.0,
                        *self,
                        array_indices.as_ptr(),
                        array_indices.len(),
                    );
                }
            }
        }
    };
}

macro_rules! declare_set_state_value_matrix {
    ( $type:ty, $func:path ) => {
        impl ConstantStateValueMatrix for $type {
            fn set_constant_state_value_matrix(
                &self,
                program_node: &mut ProgramNode,
                state_id: StateValueId,
                array_indices: &[usize],
            ) {
                unsafe {
                    $func(
                        program_node.handle,
                        state_id.0,
                        *self,
                        array_indices.as_ptr(),
                        array_indices.len(),
                    );
                }
            }
        }
    };
}

declare_set_state_value!(u8, sys::Cobalt_ProgramNode_SetConstantStateValueV1UInt8);
declare_set_state_value!(u16, sys::Cobalt_ProgramNode_SetConstantStateValueV1UInt16);
declare_set_state_value!(u32, sys::Cobalt_ProgramNode_SetConstantStateValueV1UInt32);
declare_set_state_value!(i8, sys::Cobalt_ProgramNode_SetConstantStateValueV1Int8);
declare_set_state_value!(i16, sys::Cobalt_ProgramNode_SetConstantStateValueV1Int16);
declare_set_state_value!(i32, sys::Cobalt_ProgramNode_SetConstantStateValueV1Int32);
declare_set_state_value!(f32, sys::Cobalt_ProgramNode_SetConstantStateValueV1Float32);
declare_set_state_value!(f64, sys::Cobalt_ProgramNode_SetConstantStateValueV1Float64);
declare_set_state_value!(
    &[u8; 2],
    sys::Cobalt_ProgramNode_SetConstantStateValueV2UInt8
);
declare_set_state_value!(
    &[u16; 2],
    sys::Cobalt_ProgramNode_SetConstantStateValueV2UInt16
);
declare_set_state_value!(
    &[u32; 2],
    sys::Cobalt_ProgramNode_SetConstantStateValueV2UInt32
);
declare_set_state_value!(
    &[i8; 2],
    sys::Cobalt_ProgramNode_SetConstantStateValueV2Int8
);
declare_set_state_value!(
    &[i16; 2],
    sys::Cobalt_ProgramNode_SetConstantStateValueV2Int16
);
declare_set_state_value!(
    &[i32; 2],
    sys::Cobalt_ProgramNode_SetConstantStateValueV2Int32
);
declare_set_state_value!(
    &[f32; 2],
    sys::Cobalt_ProgramNode_SetConstantStateValueV2Float32
);
declare_set_state_value!(
    &[f64; 2],
    sys::Cobalt_ProgramNode_SetConstantStateValueV2Float64
);
declare_set_state_value!(
    &[u8; 3],
    sys::Cobalt_ProgramNode_SetConstantStateValueV3UInt8
);
declare_set_state_value!(
    &[u16; 3],
    sys::Cobalt_ProgramNode_SetConstantStateValueV3UInt16
);
declare_set_state_value!(
    &[u32; 3],
    sys::Cobalt_ProgramNode_SetConstantStateValueV3UInt32
);
declare_set_state_value!(
    &[i8; 3],
    sys::Cobalt_ProgramNode_SetConstantStateValueV3Int8
);
declare_set_state_value!(
    &[i16; 3],
    sys::Cobalt_ProgramNode_SetConstantStateValueV3Int16
);
declare_set_state_value!(
    &[i32; 3],
    sys::Cobalt_ProgramNode_SetConstantStateValueV3Int32
);
declare_set_state_value!(
    &[f32; 3],
    sys::Cobalt_ProgramNode_SetConstantStateValueV3Float32
);
declare_set_state_value!(
    &[f64; 3],
    sys::Cobalt_ProgramNode_SetConstantStateValueV3Float64
);
declare_set_state_value!(
    &[u8; 4],
    sys::Cobalt_ProgramNode_SetConstantStateValueV4UInt8
);
declare_set_state_value!(
    &[u16; 4],
    sys::Cobalt_ProgramNode_SetConstantStateValueV4UInt16
);
declare_set_state_value!(
    &[u32; 4],
    sys::Cobalt_ProgramNode_SetConstantStateValueV4UInt32
);
declare_set_state_value!(
    &[i8; 4],
    sys::Cobalt_ProgramNode_SetConstantStateValueV4Int8
);
declare_set_state_value!(
    &[i16; 4],
    sys::Cobalt_ProgramNode_SetConstantStateValueV4Int16
);
declare_set_state_value!(
    &[i32; 4],
    sys::Cobalt_ProgramNode_SetConstantStateValueV4Int32
);
declare_set_state_value!(
    &[f32; 4],
    sys::Cobalt_ProgramNode_SetConstantStateValueV4Float32
);
declare_set_state_value!(
    &[f64; 4],
    sys::Cobalt_ProgramNode_SetConstantStateValueV4Float64
);

declare_set_state_value_matrix!(
    &[f32; 4],
    sys::Cobalt_ProgramNode_SetConstantStateValueM2Float32
);
declare_set_state_value_matrix!(
    &[f32; 9],
    sys::Cobalt_ProgramNode_SetConstantStateValueM3Float32
);
declare_set_state_value_matrix!(
    &[f32; 16],
    sys::Cobalt_ProgramNode_SetConstantStateValueM4Float32
);

/// Defines which shader program is used to render content
pub struct ProgramNode {
    pub(crate) handle: sys::Cobalt_ProgramNode,
    _renderer: Arc<RendererInternal>,
}

impl ProgramNode {
    pub(crate) fn new(
        handle: sys::Cobalt_ProgramNode,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        ProgramNode {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn add_child_node(&self, child_node: &StateGroupNode) {
        unsafe {
            sys::Cobalt_ProgramNode_AddChildNode(self.handle, child_node.handle);
        }
    }

    pub fn add_child_nodes(&self, child_nodes: &[&StateGroupNode]) {
        unsafe {
            let nodes: Vec<sys::Cobalt_StateGroupNode> =
                child_nodes.iter().map(|n| n.handle).collect();
            sys::Cobalt_ProgramNode_AddChildNodes(self.handle, nodes.as_ptr(), nodes.len());
        }
    }

    pub fn remove_child_node(&self, child_node: &StateGroupNode) {
        unsafe {
            sys::Cobalt_ProgramNode_RemoveChildNode(self.handle, child_node.handle);
        }
    }

    pub fn remove_child_nodes(&self, child_nodes: &[&StateGroupNode]) {
        unsafe {
            let nodes: Vec<sys::Cobalt_StateGroupNode> =
                child_nodes.iter().map(|n| n.handle).collect();
            sys::Cobalt_ProgramNode_RemoveChildNodes(self.handle, nodes.as_ptr(), nodes.len());
        }
    }

    pub fn remove_all_child_nodes(&self) {
        unsafe {
            sys::Cobalt_ProgramNode_RemoveAllChildNodes(self.handle);
        }
    }

    pub fn set_child_nodes(&self, child_nodes: &[&StateGroupNode]) {
        unsafe {
            let nodes: Vec<sys::Cobalt_StateGroupNode> =
                child_nodes.iter().map(|n| n.handle).collect();
            sys::Cobalt_ProgramNode_SetChildNodes(self.handle, nodes.as_ptr(), nodes.len());
        }
    }

    pub fn bind_shader_program(&mut self, shader_program: &ShaderProgram) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_ProgramNode_BindShaderProgram(
                self.handle,
                shader_program.handle,
            ));
        }
        Ok(())
    }

    pub fn set_constant_state_value(
        &mut self,
        state_id: StateValueId,
        value: impl ConstantStateValue,
        array_indices: Option<&[usize]>,
    ) {
        value.set_constant_state_value(self, state_id, array_indices.unwrap_or_default());
    }

    pub fn set_constant_state_value_matrix(
        &mut self,
        state_id: StateValueId,
        value: impl ConstantStateValueMatrix,
        array_indices: Option<&[usize]>,
    ) {
        value.set_constant_state_value_matrix(self, state_id, array_indices.unwrap_or_default());
    }

    pub fn reset_constant_state_value(
        &mut self,
        state_id: StateValueId,
        array_indices: Option<&[usize]>,
    ) {
        let array_indices = array_indices.unwrap_or_default();
        unsafe {
            sys::Cobalt_ProgramNode_ResetConstantStateValue(
                self.handle,
                state_id.0,
                array_indices.as_ptr(),
                array_indices.len(),
            );
        }
    }

    pub fn set_debug_name(&mut self, name: impl AsRef<str>) {
        let bytes = name.as_ref().as_bytes();
        unsafe {
            sys::Cobalt_ProgramNode_SetDebugName(
                self.handle,
                bytes.as_ptr() as *const std::ffi::c_char,
                bytes.len(),
            );
        }
    }
}

impl Drop for ProgramNode {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_ProgramNode_Delete(self.handle);
        }
    }
}

unsafe impl Send for ProgramNode {}
unsafe impl Sync for ProgramNode {}
