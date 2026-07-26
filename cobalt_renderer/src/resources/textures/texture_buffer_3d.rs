// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use num_enum::TryFromPrimitive;
use std::sync::Arc;

use super::{
    DataFormat, DataPersistenceFlags, ImageFormat, PerformanceHint, SourceDataFormat,
    SourceImageFormat, TextureBuffer, TextureUsageFlags,
};
use crate::RendererResult;
use crate::render_tree::StateContainer;
use crate::renderer::RendererInternal;
use crate::resources::TextureId;
use crate::resources::batching::TransferBatch;

use cobalt_renderer_sys as sys;

pub struct UnallocatedTextureBuffer3D<'a> {
    texture_buffer: TextureBuffer3D,
    _initial_data: std::marker::PhantomData<&'a i32>,
}

impl<'a> UnallocatedTextureBuffer3D<'a> {
    pub(crate) fn new(
        handle: sys::Cobalt_TextureBuffer3D,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        UnallocatedTextureBuffer3D {
            texture_buffer: TextureBuffer3D::new(handle, renderer_internal),
            _initial_data: std::marker::PhantomData,
        }
    }

    pub fn set_texture_format(&mut self, image_format: ImageFormat, data_format: DataFormat) {
        unsafe {
            sys::Cobalt_TextureBuffer3D_SetTextureFormat(
                self.texture_buffer.handle,
                image_format as sys::Cobalt_ImageFormat,
                data_format as sys::Cobalt_DataFormat,
            );
        }
    }

    pub fn set_texture_dimensions(
        &mut self,
        image_dimensions: &[u32; 3],
        mipmap_level_count: Option<i32>,
    ) {
        unsafe {
            sys::Cobalt_TextureBuffer3D_SetTextureDimensions(
                self.texture_buffer.handle,
                image_dimensions,
                mipmap_level_count.unwrap_or(1),
            )
        }
    }
    pub fn set_initial_data<S: Sized>(
        &mut self,
        source_buffer: &'a [S],
        image_format: SourceImageFormat,
        data_format: SourceDataFormat,
        mipmap_level: Option<i32>,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_TextureBuffer3D_SetInitialData(
                self.texture_buffer.handle,
                source_buffer.as_ptr() as *const std::ffi::c_void,
                core::mem::size_of_val(source_buffer),
                image_format as sys::Cobalt_SourceImageFormat,
                data_format as sys::Cobalt_SourceDataFormat,
                mipmap_level.unwrap_or(0),
            ))
        }
        Ok(())
    }
    pub fn set_usage_flags(&mut self, usage_flags: TextureUsageFlags) {
        unsafe {
            sys::Cobalt_TextureBuffer_SetUsageFlags(
                self.texture_buffer.handle as sys::Cobalt_TextureBuffer,
                usage_flags.bits() as sys::Cobalt_TextureUsageFlags,
            );
        }
    }

    pub fn set_performance_hints(
        &mut self,
        performance_hint_cpu: PerformanceHint,
        performance_hint_gpu: PerformanceHint,
    ) {
        unsafe {
            sys::Cobalt_TextureBuffer_SetPerformanceHints(
                self.texture_buffer.handle as sys::Cobalt_TextureBuffer,
                performance_hint_cpu.bits() as sys::Cobalt_TexturePerformanceHint,
                performance_hint_gpu.bits() as sys::Cobalt_TexturePerformanceHint,
            );
        }
    }

    pub fn set_data_persistence_flags(&mut self, data_persistence_flags: DataPersistenceFlags) {
        unsafe {
            sys::Cobalt_TextureBuffer_SetDataPersistenceFlags(
                self.texture_buffer.handle as sys::Cobalt_TextureBuffer,
                data_persistence_flags.bits() as sys::Cobalt_TextureDataPersistenceFlags,
            );
        }
    }

    pub fn allocate_memory(self) -> RendererResult<TextureBuffer3D> {
        unsafe {
            return_on_failure!(sys::Cobalt_TextureBuffer3D_AllocateMemory(
                self.texture_buffer.handle
            ))
        }
        Ok(self.texture_buffer)
    }
}

pub struct TextureBuffer3D {
    pub(crate) handle: sys::Cobalt_TextureBuffer3D,
    _renderer: Arc<RendererInternal>,
}

impl TextureBuffer3D {
    pub(crate) fn new(
        handle: sys::Cobalt_TextureBuffer3D,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        TextureBuffer3D {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn allocated_image_format(&self) -> ImageFormat {
        let value = unsafe { sys::Cobalt_TextureBuffer3D_AllocatedImageFormat(self.handle) };
        ImageFormat::try_from_primitive(value as i32).unwrap()
    }

    pub fn allocated_data_format(&self) -> DataFormat {
        let value = unsafe { sys::Cobalt_TextureBuffer3D_AllocatedDataFormat(self.handle) };
        DataFormat::try_from_primitive(value as i32).unwrap()
    }

    pub fn mipmap_level_count(&self) -> i32 {
        unsafe { sys::Cobalt_TextureBuffer3D_MipmapLevelCount(self.handle) }
    }

    pub fn mipmap_level_dimensions(&self, mipmap_level: i32) -> [u32; 3] {
        unsafe {
            let mut dimensions: [u32; 3] = [0; 3];
            sys::Cobalt_TextureBuffer3D_MipmapLevelDimensions(
                self.handle,
                mipmap_level,
                &mut dimensions,
            );
            dimensions
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn queue_data_update<S: Sized>(
        &mut self,
        source_buffer: &[S],
        image_format: SourceImageFormat,
        data_format: SourceDataFormat,
        mipmap_level: i32,
        image_offset_in_pixels: &[u32; 3],
        image_region_in_pixels: &[u32; 3],
        transfer_batch: Option<&TransferBatch>,
    ) -> RendererResult<()> {
        unsafe {
            let transfer_batch = match transfer_batch {
                Some(t) => t.handle,
                None => std::ptr::null_mut(),
            };

            return_on_failure!(sys::Cobalt_TextureBuffer3D_QueueDataUpdate(
                self.handle,
                source_buffer.as_ptr() as *const std::ffi::c_void,
                core::mem::size_of_val(source_buffer),
                image_format as sys::Cobalt_SourceImageFormat,
                data_format as sys::Cobalt_SourceDataFormat,
                mipmap_level,
                image_offset_in_pixels,
                image_region_in_pixels,
                transfer_batch,
            ))
        }
        Ok(())
    }
}

impl TextureBuffer for TextureBuffer3D {
    fn texture_handle(&self) -> sys::Cobalt_TextureBuffer {
        self.handle as sys::Cobalt_TextureBuffer
    }

    fn bind_to_state_container(
        &mut self,
        texture_id: TextureId,
        container: &mut impl StateContainer,
    ) {
        unsafe {
            sys::Cobalt_StateContainer_BindTexture3D(
                container.node_handle(),
                texture_id.0,
                self.handle,
            )
        }
    }
}

impl Drop for TextureBuffer3D {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_TextureBuffer3D_Delete(self.handle);
        }
    }
}

unsafe impl Send for TextureBuffer3D {}
unsafe impl Sync for TextureBuffer3D {}
