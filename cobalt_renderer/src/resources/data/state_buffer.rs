// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use bitflags::bitflags;
use std::sync::Arc;

use super::StateBufferLayout;
use crate::RendererResult;
use crate::renderer::RendererInternal;
use crate::resources::StateValueId;

use cobalt_renderer_sys as sys;

bitflags! {
    /// Indicates how state buffer will be used
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StateBufferPerformanceHint : u32 {
        const Default = 0x00000000;
        const ReadNever = 0x00000001;
        const ReadRarely = 0x00000002;
        const ReadOften = 0x00000004;
        const ReadFlagsMask = 0x000000FF;
        const WriteNever = 0x00000100;
        const WriteRarely = 0x00000200;
        const WriteOften = 0x00000400;
        const WriteFlagsMask = 0x0000FF00;
    }
}

// This is a workaround for Rust not having generic specialization
// which would allow us to have different functions under the same name
// that would take different input types and have different implementations.
// Instead we have a trait which we only implement on some types
// which then have specializations. Then we can have one generic
// function which takes this trait and calls the specialized function
// on the type. Not ideal but functional

/// A type which can be used to set a value in a state buffer, scalar or vector
pub trait StateBufferValue {
    #[doc(hidden)]
    fn set_state_buffer_value(
        &self,
        buffer: &mut StateBuffer,
        page_no: u32,
        state_id: StateValueId,
        array_indices: &[usize],
    );
}

/// A type which can be used to set a value in a state buffer, only matrix
pub trait StateBufferValueMatrix {
    #[doc(hidden)]
    fn set_state_buffer_value_matrix(
        &self,
        buffer: &mut StateBuffer,
        page_no: u32,
        state_id: StateValueId,
        array_indices: &[usize],
    );
}

impl StateBufferValue for bool {
    fn set_state_buffer_value(
        &self,
        buffer: &mut StateBuffer,
        page_no: u32,
        state_id: StateValueId,
        array_indices: &[usize],
    ) {
        unsafe {
            sys::Cobalt_StateBuffer_SetStateValueForPageBool(
                buffer.handle,
                page_no,
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
        impl StateBufferValue for $type {
            fn set_state_buffer_value(
                &self,
                buffer: &mut StateBuffer,
                page_no: u32,
                state_id: StateValueId,
                array_indices: &[usize],
            ) {
                unsafe {
                    $func(
                        buffer.handle,
                        page_no,
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
        impl StateBufferValueMatrix for $type {
            fn set_state_buffer_value_matrix(
                &self,
                buffer: &mut StateBuffer,
                page_no: u32,
                state_id: StateValueId,
                array_indices: &[usize],
            ) {
                unsafe {
                    $func(
                        buffer.handle,
                        page_no,
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

declare_set_state_value!(u8, sys::Cobalt_StateBuffer_SetStateValueForPageV1UInt8);
declare_set_state_value!(u16, sys::Cobalt_StateBuffer_SetStateValueForPageV1UInt16);
declare_set_state_value!(u32, sys::Cobalt_StateBuffer_SetStateValueForPageV1UInt32);
declare_set_state_value!(i8, sys::Cobalt_StateBuffer_SetStateValueForPageV1Int8);
declare_set_state_value!(i16, sys::Cobalt_StateBuffer_SetStateValueForPageV1Int16);
declare_set_state_value!(i32, sys::Cobalt_StateBuffer_SetStateValueForPageV1Int32);
declare_set_state_value!(f32, sys::Cobalt_StateBuffer_SetStateValueForPageV1Float32);
declare_set_state_value!(f64, sys::Cobalt_StateBuffer_SetStateValueForPageV1Float64);
declare_set_state_value!(
    &[u8; 2],
    sys::Cobalt_StateBuffer_SetStateValueForPageV2UInt8
);
declare_set_state_value!(
    &[u16; 2],
    sys::Cobalt_StateBuffer_SetStateValueForPageV2UInt16
);
declare_set_state_value!(
    &[u32; 2],
    sys::Cobalt_StateBuffer_SetStateValueForPageV2UInt32
);
declare_set_state_value!(&[i8; 2], sys::Cobalt_StateBuffer_SetStateValueForPageV2Int8);
declare_set_state_value!(
    &[i16; 2],
    sys::Cobalt_StateBuffer_SetStateValueForPageV2Int16
);
declare_set_state_value!(
    &[i32; 2],
    sys::Cobalt_StateBuffer_SetStateValueForPageV2Int32
);
declare_set_state_value!(
    &[f32; 2],
    sys::Cobalt_StateBuffer_SetStateValueForPageV2Float32
);
declare_set_state_value!(
    &[f64; 2],
    sys::Cobalt_StateBuffer_SetStateValueForPageV2Float64
);
declare_set_state_value!(
    &[u8; 3],
    sys::Cobalt_StateBuffer_SetStateValueForPageV3UInt8
);
declare_set_state_value!(
    &[u16; 3],
    sys::Cobalt_StateBuffer_SetStateValueForPageV3UInt16
);
declare_set_state_value!(
    &[u32; 3],
    sys::Cobalt_StateBuffer_SetStateValueForPageV3UInt32
);
declare_set_state_value!(&[i8; 3], sys::Cobalt_StateBuffer_SetStateValueForPageV3Int8);
declare_set_state_value!(
    &[i16; 3],
    sys::Cobalt_StateBuffer_SetStateValueForPageV3Int16
);
declare_set_state_value!(
    &[i32; 3],
    sys::Cobalt_StateBuffer_SetStateValueForPageV3Int32
);
declare_set_state_value!(
    &[f32; 3],
    sys::Cobalt_StateBuffer_SetStateValueForPageV3Float32
);
declare_set_state_value!(
    &[f64; 3],
    sys::Cobalt_StateBuffer_SetStateValueForPageV3Float64
);
declare_set_state_value!(
    &[u8; 4],
    sys::Cobalt_StateBuffer_SetStateValueForPageV4UInt8
);
declare_set_state_value!(
    &[u16; 4],
    sys::Cobalt_StateBuffer_SetStateValueForPageV4UInt16
);
declare_set_state_value!(
    &[u32; 4],
    sys::Cobalt_StateBuffer_SetStateValueForPageV4UInt32
);
declare_set_state_value!(&[i8; 4], sys::Cobalt_StateBuffer_SetStateValueForPageV4Int8);
declare_set_state_value!(
    &[i16; 4],
    sys::Cobalt_StateBuffer_SetStateValueForPageV4Int16
);
declare_set_state_value!(
    &[i32; 4],
    sys::Cobalt_StateBuffer_SetStateValueForPageV4Int32
);
declare_set_state_value!(
    &[f32; 4],
    sys::Cobalt_StateBuffer_SetStateValueForPageV4Float32
);
declare_set_state_value!(
    &[f64; 4],
    sys::Cobalt_StateBuffer_SetStateValueForPageV4Float64
);

declare_set_state_value_matrix!(
    &[f32; 4],
    sys::Cobalt_StateBuffer_SetStateValueForPageM2Float32
);
declare_set_state_value_matrix!(
    &[f32; 9],
    sys::Cobalt_StateBuffer_SetStateValueForPageM3Float32
);
declare_set_state_value_matrix!(
    &[f32; 16],
    sys::Cobalt_StateBuffer_SetStateValueForPageM4Float32
);

pub struct StateBuffer {
    pub(crate) handle: sys::Cobalt_StateBuffer,
    _renderer: Arc<RendererInternal>,
}

impl StateBuffer {
    pub(crate) fn new(
        handle: sys::Cobalt_StateBuffer,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        StateBuffer {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn allocate_memory(&mut self) -> RendererResult<()> {
        unsafe { return_on_failure!(sys::Cobalt_StateBuffer_AllocateMemory(self.handle)) }
        Ok(())
    }

    pub fn set_performance_hints(
        &mut self,
        performance_hint_cpu: StateBufferPerformanceHint,
        performance_hint_gpu: StateBufferPerformanceHint,
    ) {
        unsafe {
            sys::Cobalt_StateBuffer_SetPerformanceHints(
                self.handle,
                performance_hint_cpu.bits() as sys::Cobalt_StateBufferPerformanceHint,
                performance_hint_gpu.bits() as sys::Cobalt_StateBufferPerformanceHint,
            )
        }
    }

    pub fn set_manual_page_size(&mut self, page_size_in_bytes: usize) {
        unsafe { sys::Cobalt_StateBuffer_SetManualPageSize(self.handle, page_size_in_bytes) }
    }

    pub fn bind_buffer_layout(
        &mut self,
        state_buffer_layout: &mut StateBufferLayout,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_StateBuffer_BindBufferLayout(
                self.handle,
                state_buffer_layout.handle,
            ))
        }
        Ok(())
    }

    pub fn set_page_settings(&mut self, initial_page_count: u32, allow_dynamic_resize: bool) {
        unsafe {
            sys::Cobalt_StateBuffer_SetPageSettings(
                self.handle,
                initial_page_count,
                if allow_dynamic_resize { 1 } else { 0 },
            )
        }
    }

    pub fn resize_page_count(&mut self, page_count: u32) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_StateBuffer_ResizePageCount(
                self.handle,
                page_count,
            ))
        }
        Ok(())
    }

    pub fn state_value_id(&mut self, name: impl AsRef<str>) -> Option<StateValueId> {
        let name = name.as_ref();
        unsafe {
            match sys::Cobalt_StateBuffer_GetStateValueId(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
            ) {
                0xFFFFFFFF => None,
                n => Some(StateValueId(n)),
            }
        }
    }

    pub fn set_state_value(
        &mut self,
        state_id: StateValueId,
        value: impl StateBufferValue,
        array_indices: Option<&[usize]>,
        page_no: Option<u32>,
    ) {
        value.set_state_buffer_value(
            self,
            page_no.unwrap_or(0),
            state_id,
            array_indices.unwrap_or(&[]),
        );
    }

    pub fn set_state_value_matrix(
        &mut self,
        state_id: StateValueId,
        value: impl StateBufferValueMatrix,
        array_indices: Option<&[usize]>,
        page_no: Option<u32>,
    ) {
        value.set_state_buffer_value_matrix(
            self,
            page_no.unwrap_or(0),
            state_id,
            array_indices.unwrap_or(&[]),
        );
    }

    pub fn set_raw_page_data<S: Sized>(
        &mut self,
        data: &[S],
        data_offset_in_bytes: usize,
        page_no: Option<u32>,
    ) {
        unsafe {
            sys::Cobalt_StateBuffer_SetRawPageData(
                self.handle,
                page_no.unwrap_or(0),
                data.as_ptr() as *const u8,
                core::mem::size_of_val(data),
                data_offset_in_bytes,
            )
        }
    }
}

impl Drop for StateBuffer {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_StateBuffer_Delete(self.handle);
        }
    }
}

unsafe impl Send for StateBuffer {}
unsafe impl Sync for StateBuffer {}
