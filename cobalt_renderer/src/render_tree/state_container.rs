// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use crate::resources::{
    ResourceArrayId, SamplerId, StateBufferId, StateValueId, TextureId,
    data::{DataArray, StateBuffer, TexelArray},
    textures::*,
};

use cobalt_renderer_sys as sys;

// This is a workaround for Rust not having generic specialization
// which would allow us to have different functions under the same name
// that would take different input types and have different implementations.
// Instead we have a trait which we only implement on some types
// which then have specializations. Then we can have one generic
// function which takes this trait and calls the specialized function
// on the type. Not ideal but functional

pub trait StateValue {
    #[doc(hidden)]
    fn set_state_value(
        &self,
        container: &mut impl StateContainer,
        state_id: StateValueId,
        array_indices: &[usize],
    );
}

pub trait StateValueMatrix {
    #[doc(hidden)]
    fn set_state_value_matrix(
        &self,
        container: &mut impl StateContainer,
        state_id: StateValueId,
        array_indices: &[usize],
    );
}

impl StateValue for bool {
    fn set_state_value(
        &self,
        container: &mut impl StateContainer,
        state_id: StateValueId,
        array_indices: &[usize],
    ) {
        unsafe {
            sys::Cobalt_StateContainer_SetStateValueBool(
                container.node_handle(),
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
        impl StateValue for $type {
            fn set_state_value(
                &self,
                container: &mut impl StateContainer,
                state_id: StateValueId,
                array_indices: &[usize],
            ) {
                let handle = container.node_handle() as sys::Cobalt_StateContainer;
                unsafe {
                    $func(
                        handle,
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
        impl StateValueMatrix for $type {
            fn set_state_value_matrix(
                &self,
                container: &mut impl StateContainer,
                state_id: StateValueId,
                array_indices: &[usize],
            ) {
                let handle = container.node_handle() as sys::Cobalt_StateContainer;
                unsafe {
                    $func(
                        handle,
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

declare_set_state_value!(u8, sys::Cobalt_StateContainer_SetStateValueV1UInt8);
declare_set_state_value!(u16, sys::Cobalt_StateContainer_SetStateValueV1UInt16);
declare_set_state_value!(u32, sys::Cobalt_StateContainer_SetStateValueV1UInt32);
declare_set_state_value!(i8, sys::Cobalt_StateContainer_SetStateValueV1Int8);
declare_set_state_value!(i16, sys::Cobalt_StateContainer_SetStateValueV1Int16);
declare_set_state_value!(i32, sys::Cobalt_StateContainer_SetStateValueV1Int32);
declare_set_state_value!(f32, sys::Cobalt_StateContainer_SetStateValueV1Float32);
declare_set_state_value!(f64, sys::Cobalt_StateContainer_SetStateValueV1Float64);
declare_set_state_value!(&[u8; 2], sys::Cobalt_StateContainer_SetStateValueV2UInt8);
declare_set_state_value!(&[u16; 2], sys::Cobalt_StateContainer_SetStateValueV2UInt16);
declare_set_state_value!(&[u32; 2], sys::Cobalt_StateContainer_SetStateValueV2UInt32);
declare_set_state_value!(&[i8; 2], sys::Cobalt_StateContainer_SetStateValueV2Int8);
declare_set_state_value!(&[i16; 2], sys::Cobalt_StateContainer_SetStateValueV2Int16);
declare_set_state_value!(&[i32; 2], sys::Cobalt_StateContainer_SetStateValueV2Int32);
declare_set_state_value!(&[f32; 2], sys::Cobalt_StateContainer_SetStateValueV2Float32);
declare_set_state_value!(&[f64; 2], sys::Cobalt_StateContainer_SetStateValueV2Float64);
declare_set_state_value!(&[u8; 3], sys::Cobalt_StateContainer_SetStateValueV3UInt8);
declare_set_state_value!(&[u16; 3], sys::Cobalt_StateContainer_SetStateValueV3UInt16);
declare_set_state_value!(&[u32; 3], sys::Cobalt_StateContainer_SetStateValueV3UInt32);
declare_set_state_value!(&[i8; 3], sys::Cobalt_StateContainer_SetStateValueV3Int8);
declare_set_state_value!(&[i16; 3], sys::Cobalt_StateContainer_SetStateValueV3Int16);
declare_set_state_value!(&[i32; 3], sys::Cobalt_StateContainer_SetStateValueV3Int32);
declare_set_state_value!(&[f32; 3], sys::Cobalt_StateContainer_SetStateValueV3Float32);
declare_set_state_value!(&[f64; 3], sys::Cobalt_StateContainer_SetStateValueV3Float64);
declare_set_state_value!(&[u8; 4], sys::Cobalt_StateContainer_SetStateValueV4UInt8);
declare_set_state_value!(&[u16; 4], sys::Cobalt_StateContainer_SetStateValueV4UInt16);
declare_set_state_value!(&[u32; 4], sys::Cobalt_StateContainer_SetStateValueV4UInt32);
declare_set_state_value!(&[i8; 4], sys::Cobalt_StateContainer_SetStateValueV4Int8);
declare_set_state_value!(&[i16; 4], sys::Cobalt_StateContainer_SetStateValueV4Int16);
declare_set_state_value!(&[i32; 4], sys::Cobalt_StateContainer_SetStateValueV4Int32);
declare_set_state_value!(&[f32; 4], sys::Cobalt_StateContainer_SetStateValueV4Float32);
declare_set_state_value!(&[f64; 4], sys::Cobalt_StateContainer_SetStateValueV4Float64);

declare_set_state_value_matrix!(&[f32; 4], sys::Cobalt_StateContainer_SetStateValueM2Float32);
declare_set_state_value_matrix!(&[f32; 9], sys::Cobalt_StateContainer_SetStateValueM3Float32);
declare_set_state_value_matrix!(
    &[f32; 16],
    sys::Cobalt_StateContainer_SetStateValueM4Float32
);

pub trait StateContainer: Sized {
    #[doc(hidden)]
    fn node_handle(&mut self) -> sys::Cobalt_StateContainer;

    fn set_state_value(
        &mut self,
        state_id: StateValueId,
        value: impl StateValue,
        array_indices: Option<&[usize]>,
    ) {
        value.set_state_value(self, state_id, array_indices.unwrap_or_default());
    }

    fn set_state_value_matrix(
        &mut self,
        state_id: StateValueId,
        value: impl StateValueMatrix,
        array_indices: Option<&[usize]>,
    ) {
        value.set_state_value_matrix(self, state_id, array_indices.unwrap_or_default());
    }

    fn bind_texture_with_combined_sampler_2d(
        &mut self,
        texture_id: TextureId,
        texture: &mut TextureBuffer2D,
        sampler: &mut TextureSampler2D,
    ) {
        unsafe {
            sys::Cobalt_StateContainer_BindTextureWithCombinedSampler2D(
                self.node_handle() as sys::Cobalt_StateContainer,
                texture_id.0,
                texture.handle,
                sampler.handle,
            )
        }
    }

    fn bind_texture_with_combined_sampler_1d(
        &mut self,
        texture_id: TextureId,
        texture: &mut TextureBuffer1D,
        sampler: &mut TextureSampler1D,
    ) {
        unsafe {
            sys::Cobalt_StateContainer_BindTextureWithCombinedSampler1D(
                self.node_handle() as sys::Cobalt_StateContainer,
                texture_id.0,
                texture.handle,
                sampler.handle,
            )
        }
    }
    fn bind_texture_with_combined_sampler_3d(
        &mut self,
        texture_id: TextureId,
        texture: &mut TextureBuffer3D,
        sampler: &mut TextureSampler3D,
    ) {
        unsafe {
            sys::Cobalt_StateContainer_BindTextureWithCombinedSampler3D(
                self.node_handle() as sys::Cobalt_StateContainer,
                texture_id.0,
                texture.handle,
                sampler.handle,
            )
        }
    }
    fn bind_texture_with_combined_sampler_cube(
        &mut self,
        texture_id: TextureId,
        texture: &mut TextureBufferCube,
        sampler: &mut TextureSamplerCube,
    ) {
        unsafe {
            sys::Cobalt_StateContainer_BindTextureWithCombinedSamplerCube(
                self.node_handle() as sys::Cobalt_StateContainer,
                texture_id.0,
                texture.handle,
                sampler.handle,
            )
        }
    }
    fn bind_texture_with_combined_sampler_1d_array(
        &mut self,
        texture_id: TextureId,
        texture: &mut TextureBuffer1DArray,
        sampler: &mut TextureSampler1DArray,
    ) {
        unsafe {
            sys::Cobalt_StateContainer_BindTextureWithCombinedSampler1DArray(
                self.node_handle() as sys::Cobalt_StateContainer,
                texture_id.0,
                texture.handle,
                sampler.handle,
            )
        }
    }
    fn bind_texture_with_combined_sampler_2d_array(
        &mut self,
        texture_id: TextureId,
        texture: &mut TextureBuffer2DArray,
        sampler: &mut TextureSampler2DArray,
    ) {
        unsafe {
            sys::Cobalt_StateContainer_BindTextureWithCombinedSampler2DArray(
                self.node_handle() as sys::Cobalt_StateContainer,
                texture_id.0,
                texture.handle,
                sampler.handle,
            )
        }
    }
    fn bind_texture_with_combined_sampler_cube_array(
        &mut self,
        texture_id: TextureId,
        texture: &mut TextureBufferCubeArray,
        sampler: &mut TextureSamplerCubeArray,
    ) {
        unsafe {
            sys::Cobalt_StateContainer_BindTextureWithCombinedSamplerCubeArray(
                self.node_handle() as sys::Cobalt_StateContainer,
                texture_id.0,
                texture.handle,
                sampler.handle,
            )
        }
    }
    fn bind_texture(&mut self, texture_id: TextureId, texture: &mut impl TextureBuffer) {
        texture.bind_to_state_container(texture_id, self)
    }

    fn unbind_texture(&mut self, texture_id: TextureId) {
        unsafe {
            sys::Cobalt_StateContainer_UnbindTexture(
                self.node_handle() as sys::Cobalt_StateContainer,
                texture_id.0,
            )
        }
    }

    fn bind_sampler(&mut self, sampler_id: SamplerId, sampler: &mut impl TextureSampler) {
        sampler.bind_to_state_container(sampler_id, self)
    }

    fn unbind_sampler(&mut self, sampler_id: SamplerId) {
        unsafe {
            sys::Cobalt_StateContainer_UnbindSampler(
                self.node_handle() as sys::Cobalt_StateContainer,
                sampler_id.0,
            )
        }
    }

    fn bind_state_buffer(
        &mut self,
        state_buffer_id: StateBufferId,
        state_buffer: &mut StateBuffer,
        state_buffer_page_no: Option<u32>,
    ) {
        unsafe {
            sys::Cobalt_StateContainer_BindStateBuffer(
                self.node_handle() as sys::Cobalt_StateContainer,
                state_buffer_id.0,
                state_buffer.handle,
                state_buffer_page_no.unwrap_or(0),
            )
        }
    }

    fn unbind_state_buffer(&mut self, state_buffer_id: StateBufferId) {
        unsafe {
            sys::Cobalt_StateContainer_UnbindStateBuffer(
                self.node_handle() as sys::Cobalt_StateContainer,
                state_buffer_id.0,
            )
        }
    }

    fn bind_data_array(
        &mut self,
        resource_array_id: ResourceArrayId,
        data_array: &mut DataArray,
        reset_counter: bool,
    ) {
        unsafe {
            sys::Cobalt_StateContainer_BindDataArray(
                self.node_handle() as sys::Cobalt_StateContainer,
                resource_array_id.0,
                data_array.handle,
                if reset_counter { 1 } else { 0 },
            )
        }
    }

    fn bind_texel_array(
        &mut self,
        resource_array_id: ResourceArrayId,
        texel_array: &mut TexelArray,
    ) {
        unsafe {
            sys::Cobalt_StateContainer_BindTexelArray(
                self.node_handle() as sys::Cobalt_StateContainer,
                resource_array_id.0,
                texel_array.handle,
            )
        }
    }

    fn unbind_resource_array(&mut self, resource_array_id: ResourceArrayId) {
        unsafe {
            sys::Cobalt_StateContainer_UnbindResourceArray(
                self.node_handle() as sys::Cobalt_StateContainer,
                resource_array_id.0,
            )
        }
    }
}
