// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License
use num_enum::TryFromPrimitive;
use std::sync::Arc;

use super::{DataFormat, ImageFormat, SourceDataFormat, SourceImageFormat, TextureBuffer};
use crate::render_tree::StateContainer;
use crate::renderer::RendererInternal;
use crate::resources::TextureId;
use crate::resources::batching::TransferBatch;
use crate::{RendererError, RendererResult};

use cobalt_renderer_sys as sys;

/// Series of 1D image textures on GPU
pub struct TextureBuffer1DArray {
    pub(crate) handle: sys::Cobalt_TextureBuffer1DArray,
    _renderer: Arc<RendererInternal>,
}

impl TextureBuffer1DArray {
    pub(crate) fn new(
        handle: sys::Cobalt_TextureBuffer1DArray,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        TextureBuffer1DArray {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn allocate_memory(&mut self) -> RendererResult<()> {
        unsafe { return_on_failure!(sys::Cobalt_TextureBuffer1DArray_AllocateMemory(self.handle)) }
        Ok(())
    }

    pub fn set_texture_format(&mut self, image_format: ImageFormat, data_format: DataFormat) {
        unsafe {
            sys::Cobalt_TextureBuffer1DArray_SetTextureFormat(
                self.handle,
                image_format as sys::Cobalt_ImageFormat,
                data_format as sys::Cobalt_DataFormat,
            );
        }
    }

    pub fn set_texture_dimensions(
        &mut self,
        image_dimensions: u32,
        array_size: usize,
        mipmap_level_count: Option<i32>,
    ) {
        unsafe {
            sys::Cobalt_TextureBuffer1DArray_SetTextureDimensions(
                self.handle,
                image_dimensions,
                array_size,
                mipmap_level_count.unwrap_or(1),
            )
        }
    }

    pub fn allocated_image_format(&self) -> ImageFormat {
        let value = unsafe { sys::Cobalt_TextureBuffer1DArray_AllocatedImageFormat(self.handle) };
        ImageFormat::try_from_primitive(value as i32).unwrap()
    }

    pub fn allocated_data_format(&self) -> DataFormat {
        let value = unsafe { sys::Cobalt_TextureBuffer1DArray_AllocatedDataFormat(self.handle) };
        DataFormat::try_from_primitive(value as i32).unwrap()
    }

    pub fn mipmap_level_count(&self) -> i32 {
        unsafe { sys::Cobalt_TextureBuffer1DArray_MipmapLevelCount(self.handle) }
    }

    pub fn mipmap_level_dimensions(&self, mipmap_level: i32) -> u32 {
        unsafe {
            let mut dimensions: u32 = 0;
            sys::Cobalt_TextureBuffer1DArray_MipmapLevelDimensions(
                self.handle,
                mipmap_level,
                &mut dimensions,
            );
            dimensions
        }
    }

    pub fn set_initial_data<S: Sized>(
        &mut self,
        source_buffer: &[S],
        image_format: SourceImageFormat,
        data_format: SourceDataFormat,
        array_index: usize,
        mipmap_level: Option<i32>,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_TextureBuffer1DArray_SetInitialData(
                self.handle,
                source_buffer.as_ptr() as *const std::ffi::c_void,
                core::mem::size_of_val(source_buffer),
                image_format as sys::Cobalt_SourceImageFormat,
                data_format as sys::Cobalt_SourceDataFormat,
                array_index,
                mipmap_level.unwrap_or(0),
            ))
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn queue_data_update<S: Sized>(
        &mut self,
        source_buffer: &[S],
        image_format: SourceImageFormat,
        data_format: SourceDataFormat,
        array_index: usize,
        mipmap_level: i32,
        image_offset_in_pixels: u32,
        image_region_in_pixels: u32,
        transfer_batch: Option<&TransferBatch>,
    ) -> RendererResult<()> {
        unsafe {
            let transfer_batch = match transfer_batch {
                Some(t) => t.handle,
                None => std::ptr::null_mut(),
            };

            return_on_failure!(sys::Cobalt_TextureBuffer1DArray_QueueDataUpdate(
                self.handle,
                source_buffer.as_ptr() as *const std::ffi::c_void,
                core::mem::size_of_val(source_buffer),
                image_format as sys::Cobalt_SourceImageFormat,
                data_format as sys::Cobalt_SourceDataFormat,
                array_index,
                mipmap_level,
                image_offset_in_pixels,
                image_region_in_pixels,
                transfer_batch,
            ))
        }
        Ok(())
    }
}

impl TextureBuffer for TextureBuffer1DArray {
    fn texture_handle(&self) -> sys::Cobalt_TextureBuffer {
        self.handle as sys::Cobalt_TextureBuffer
    }

    fn bind_to_state_container(&self, texture_id: TextureId, container: &mut impl StateContainer) {
        unsafe {
            sys::Cobalt_StateContainer_BindTexture1DArray(
                container.node_handle(),
                texture_id.0,
                self.handle,
            )
        }
    }
}

impl Drop for TextureBuffer1DArray {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_TextureBuffer1DArray_Delete(self.handle);
        }
    }
}

unsafe impl Send for TextureBuffer1DArray {}
unsafe impl Sync for TextureBuffer1DArray {}
