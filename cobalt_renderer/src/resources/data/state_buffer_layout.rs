// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use std::sync::Arc;

use crate::renderer::RendererInternal;
use crate::{RendererError, RendererResult};

use cobalt_renderer_sys as sys;

/// Data types for state buffer fields
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateBufferDataType {
    Null = sys::Cobalt_StateBufferDataType_Null as i32,
    Boolean = sys::Cobalt_StateBufferDataType_Boolean as i32,
    Int32 = sys::Cobalt_StateBufferDataType_Int32 as i32,
    UInt32 = sys::Cobalt_StateBufferDataType_UInt32 as i32,
    Float32 = sys::Cobalt_StateBufferDataType_Float32 as i32,
    Float64 = sys::Cobalt_StateBufferDataType_Float64 as i32,
}

/// Layout of a state value buffer in a shader
pub struct StateBufferLayout {
    pub(crate) handle: sys::Cobalt_StateBufferLayout,
    _renderer: Arc<RendererInternal>,
}

impl StateBufferLayout {
    pub(crate) fn new(
        handle: sys::Cobalt_StateBufferLayout,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        StateBufferLayout {
            handle,
            _renderer: renderer_internal,
        }
    }

    // TODO(DTM):This is a prime candidate for a builder pattern

    pub fn begin_layout_definition(&mut self) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_StateBufferLayout_BeginLayoutDefinition(
                self.handle,
            ))
        }
        Ok(())
    }

    pub fn append_field(
        &mut self,
        name: impl AsRef<str>,
        data_type: StateBufferDataType,
        array_size: usize,
    ) {
        let name = name.as_ref();
        unsafe {
            sys::Cobalt_StateBufferLayout_AppendField(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
                data_type as sys::Cobalt_StateBufferDataType,
                array_size,
            )
        }
    }

    pub fn append_vector(
        &mut self,
        name: impl AsRef<str>,
        data_type: StateBufferDataType,
        element_count: usize,
        array_size: usize,
    ) {
        let name = name.as_ref();
        unsafe {
            sys::Cobalt_StateBufferLayout_AppendVector(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
                data_type as sys::Cobalt_StateBufferDataType,
                element_count,
                array_size,
            )
        }
    }

    pub fn append_matrix(
        &mut self,
        name: impl AsRef<str>,
        data_type: StateBufferDataType,
        width: usize,
        height: usize,
        array_size: usize,
    ) {
        let name = name.as_ref();
        unsafe {
            sys::Cobalt_StateBufferLayout_AppendMatrix(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
                data_type as sys::Cobalt_StateBufferDataType,
                width,
                height,
                array_size,
            )
        }
    }

    pub fn construct_layout_definition(&mut self) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_StateBufferLayout_ConstructStateLayout(
                self.handle,
            ))
        }
        Ok(())
    }
}

impl Drop for StateBufferLayout {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_StateBufferLayout_Delete(self.handle);
        }
    }
}

unsafe impl Send for StateBufferLayout {}
unsafe impl Sync for StateBufferLayout {}
