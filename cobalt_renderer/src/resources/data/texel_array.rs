// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License
use bitflags::bitflags;
use num_enum::TryFromPrimitive;
use std::sync::Arc;

use super::TexelArrayOutput;
use crate::renderer::RendererInternal;
use crate::resources::batching::TransferBatch;
use crate::{RendererError, RendererResult};

use cobalt_renderer_sys as sys;

const fn equivalent_image_formats(source_format: SourceImageFormat, format: ImageFormat) -> bool {
    match source_format {
        SourceImageFormat::R | SourceImageFormat::X => {
            matches!(format, ImageFormat::R | ImageFormat::X)
        }
        SourceImageFormat::RG | SourceImageFormat::XY => {
            matches!(format, ImageFormat::RG | ImageFormat::XY)
        }
        SourceImageFormat::RGBA | SourceImageFormat::XYZW => {
            matches!(format, ImageFormat::RGBA | ImageFormat::XYZW)
        }
    }
}

const fn equivalent_data_formats(source_format: SourceDataFormat, format: DataFormat) -> bool {
    match source_format {
        SourceDataFormat::Int8 => matches!(format, DataFormat::Int8),
        SourceDataFormat::Int16 => matches!(format, DataFormat::Int16),
        SourceDataFormat::Int32 => matches!(format, DataFormat::Int32),
        SourceDataFormat::UNorm8 | SourceDataFormat::UInt8 => {
            matches!(format, DataFormat::UInt8 | DataFormat::UNorm8)
        }
        SourceDataFormat::UNorm16 | SourceDataFormat::UInt16 => {
            matches!(format, DataFormat::UInt16)
        }
        SourceDataFormat::UInt32 => matches!(format, DataFormat::UInt32),
        SourceDataFormat::Norm8 => matches!(format, DataFormat::Norm8),
        SourceDataFormat::Float16 => matches!(format, DataFormat::Float16),
        SourceDataFormat::Float32 => matches!(format, DataFormat::Float32),
        _ => false,
    }
}

/// GPU element format for each pixel in the texel array
#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    R = sys::Cobalt_TexelArrayImageFormat_R as i32,
    RG = sys::Cobalt_TexelArrayImageFormat_RG as i32,
    RGBA = sys::Cobalt_TexelArrayImageFormat_RGBA as i32,
    X = sys::Cobalt_TexelArrayImageFormat_X as i32,
    XY = sys::Cobalt_TexelArrayImageFormat_XY as i32,
    XYZW = sys::Cobalt_TexelArrayImageFormat_XYZW as i32,
}

impl ImageFormat {
    pub const fn element_count_per_pixel(&self) -> usize {
        match self {
            Self::R | Self::X => 1,
            Self::RG | Self::XY => 2,
            Self::RGBA | Self::XYZW => 4,
        }
    }

    pub const fn binary_equivalent_to_source(&self, format: SourceImageFormat) -> bool {
        equivalent_image_formats(format, *self)
    }

    pub const fn binary_equivalent_to(&self, format: ImageFormat) -> bool {
        match self {
            Self::R | Self::X => matches!(format, ImageFormat::R | ImageFormat::X),
            Self::RG | Self::XY => matches!(format, ImageFormat::RG | ImageFormat::XY),
            Self::RGBA | Self::XYZW => matches!(format, ImageFormat::RGBA | ImageFormat::XYZW),
        }
    }
}

/// GPU data format for each element in each pixel of the texel array
#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    Int8 = sys::Cobalt_TexelArrayDataFormat_Int8 as i32,
    Int16 = sys::Cobalt_TexelArrayDataFormat_Int16 as i32,
    Int32 = sys::Cobalt_TexelArrayDataFormat_Int32 as i32,
    UInt8 = sys::Cobalt_TexelArrayDataFormat_UInt8 as i32,
    UInt16 = sys::Cobalt_TexelArrayDataFormat_UInt16 as i32,
    UInt32 = sys::Cobalt_TexelArrayDataFormat_UInt32 as i32,
    Norm8 = sys::Cobalt_TexelArrayDataFormat_Norm8 as i32,
    UNorm8 = sys::Cobalt_TexelArrayDataFormat_UNorm8 as i32,
    Float16 = sys::Cobalt_TexelArrayDataFormat_Float16 as i32,
    Float32 = sys::Cobalt_TexelArrayDataFormat_Float32 as i32,
}

impl DataFormat {
    pub const fn bytes_per_element(&self) -> usize {
        match self {
            Self::Int8 | Self::UInt8 | Self::Norm8 | Self::UNorm8 => 1,
            Self::Int16 | Self::UInt16 | Self::Float16 => 2,
            Self::Int32 | Self::UInt32 | Self::Float32 => 4,
        }
    }

    pub const fn binary_equivalent_to_source(&self, format: SourceDataFormat) -> bool {
        equivalent_data_formats(format, *self)
    }

    pub const fn binary_equivalent_to(&self, format: DataFormat) -> bool {
        match self {
            Self::UInt8 | Self::UNorm8 => matches!(format, DataFormat::UInt8 | DataFormat::UNorm8),
            // Eq is not const so we can't do a direct comparison, so cast down to u32 to compare
            // Maybe in future it will be and we can compare directly
            // (https://stackoverflow.com/questions/60125657/rust-cant-use-enum-in-const-fn)
            f => (*f as u32) == (format as u32),
        }
    }
}

/// CPU side element format for each pixel in the texel array
#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceImageFormat {
    R = sys::Cobalt_TexelArraySourceImageFormat_R as i32,
    RG = sys::Cobalt_TexelArraySourceImageFormat_RG as i32,
    RGBA = sys::Cobalt_TexelArraySourceImageFormat_RGBA as i32,
    X = sys::Cobalt_TexelArraySourceImageFormat_X as i32,
    XY = sys::Cobalt_TexelArraySourceImageFormat_XY as i32,
    XYZW = sys::Cobalt_TexelArraySourceImageFormat_XYZW as i32,
}

impl SourceImageFormat {
    pub const fn element_count_per_pixel(&self) -> usize {
        match self {
            Self::R | Self::X => 1,
            Self::RG | Self::XY => 2,
            Self::RGBA | Self::XYZW => 4,
        }
    }

    pub const fn binary_equivalent_to(&self, format: ImageFormat) -> bool {
        equivalent_image_formats(*self, format)
    }
}

/// CPU side data format for each element in each pixel of the texel array
#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceDataFormat {
    Int8 = sys::Cobalt_TexelArraySourceDataFormat_Int8 as i32,
    Int16 = sys::Cobalt_TexelArraySourceDataFormat_Int16 as i32,
    Int32 = sys::Cobalt_TexelArraySourceDataFormat_Int32 as i32,
    UInt8 = sys::Cobalt_TexelArraySourceDataFormat_UInt8 as i32,
    UInt16 = sys::Cobalt_TexelArraySourceDataFormat_UInt16 as i32,
    UInt32 = sys::Cobalt_TexelArraySourceDataFormat_UInt32 as i32,
    Norm8 = sys::Cobalt_TexelArraySourceDataFormat_Norm8 as i32,
    Norm16 = sys::Cobalt_TexelArraySourceDataFormat_Norm16 as i32,
    Norm32 = sys::Cobalt_TexelArraySourceDataFormat_Norm32 as i32,
    UNorm8 = sys::Cobalt_TexelArraySourceDataFormat_UNorm8 as i32,
    UNorm16 = sys::Cobalt_TexelArraySourceDataFormat_UNorm16 as i32,
    UNorm32 = sys::Cobalt_TexelArraySourceDataFormat_UNorm32 as i32,
    Float16 = sys::Cobalt_TexelArraySourceDataFormat_Float16 as i32,
    Float32 = sys::Cobalt_TexelArraySourceDataFormat_Float32 as i32,
}

impl SourceDataFormat {
    pub const fn bytes_per_element(&self) -> usize {
        match self {
            Self::Int8 | Self::UInt8 | Self::Norm8 | Self::UNorm8 => 1,
            Self::Int16 | Self::UInt16 | Self::Norm16 | Self::UNorm16 | Self::Float16 => 2,
            Self::Int32 | Self::UInt32 | Self::Norm32 | Self::UNorm32 | Self::Float32 => 4,
        }
    }

    pub const fn binary_equivalent_to(&self, format: DataFormat) -> bool {
        equivalent_data_formats(*self, format)
    }
}

bitflags! {
    /// Specifies how a texel array will be used
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TexelArrayUsageFlags : u32 {
        const Default = sys::Cobalt_TexelArrayUsageFlags_Default as u32;
        const ShaderInput = sys::Cobalt_TexelArrayUsageFlags_ShaderInput as u32;
        const ShaderOutput = sys::Cobalt_TexelArrayUsageFlags_ShaderOutput as u32;
        const TransferSource = sys::Cobalt_TexelArrayUsageFlags_TransferSource as u32;
        const TransferDestination = sys::Cobalt_TexelArrayUsageFlags_TransferDestination as u32;
    }
}

/// GPU buffer for shader input/output, better for large data and image data
pub struct TexelArray {
    pub(crate) handle: sys::Cobalt_TexelArray,
    _renderer: Arc<RendererInternal>,
}

impl TexelArray {
    pub(crate) fn new(
        handle: sys::Cobalt_TexelArray,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        TexelArray {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn allocate_memory(&mut self) -> RendererResult<()> {
        unsafe { return_on_failure!(sys::Cobalt_TexelArray_AllocateMemory(self.handle)) }
        Ok(())
    }

    pub fn set_buffer_layout(
        &mut self,
        image_format: ImageFormat,
        data_format: DataFormat,
        entry_count: usize,
    ) {
        unsafe {
            sys::Cobalt_TexelArray_SetBufferLayout(
                self.handle,
                image_format as sys::Cobalt_TexelArrayImageFormat,
                data_format as sys::Cobalt_TexelArrayDataFormat,
                entry_count,
            )
        }
    }

    pub fn set_usage_flags(&mut self, usage_flags: TexelArrayUsageFlags) {
        unsafe {
            sys::Cobalt_TexelArray_SetUsageFlags(
                self.handle,
                usage_flags.bits() as sys::Cobalt_TexelArrayUsageFlags,
            )
        }
    }

    pub fn set_initial_data<S: Sized>(
        &mut self,
        source_buffer: &[S],
        image_format: SourceImageFormat,
        data_format: SourceDataFormat,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_TexelArray_SetInitialData(
                self.handle,
                source_buffer.as_ptr() as *const std::ffi::c_void,
                core::mem::size_of_val(source_buffer),
                image_format as sys::Cobalt_TexelArraySourceImageFormat,
                data_format as sys::Cobalt_TexelArraySourceDataFormat,
            ))
        }
        Ok(())
    }

    pub fn queue_data_update<S: Sized>(
        &mut self,
        source_buffer: &[S],
        image_format: SourceImageFormat,
        data_format: SourceDataFormat,
        target_buffer_offset: usize,
        transfer_batch: TransferBatch,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_TexelArray_QueueDataUpdate(
                self.handle,
                source_buffer.as_ptr() as *const std::ffi::c_void,
                core::mem::size_of_val(source_buffer),
                image_format as sys::Cobalt_TexelArraySourceImageFormat,
                data_format as sys::Cobalt_TexelArraySourceDataFormat,
                target_buffer_offset,
                transfer_batch.handle,
            ))
        }
        Ok(())
    }

    pub fn queue_data_transfer(
        &mut self,
        target_buffer: &mut TexelArray,
        transfer_count: usize,
        source_buffer_offset: usize,
        target_buffer_offset: usize,
        transfer_batch: TransferBatch,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_TexelArray_QueueDataTransfer(
                self.handle,
                target_buffer.handle,
                transfer_count,
                source_buffer_offset,
                target_buffer_offset,
                transfer_batch.handle,
            ))
        }
        Ok(())
    }

    pub fn add_output_capture_target(&mut self, output: &TexelArrayOutput) {
        unsafe { sys::Cobalt_TexelArray_AddOutputCaptureTarget(self.handle, output.handle) }
    }

    pub fn remove_output_capture_target(&mut self, output: &TexelArrayOutput) {
        unsafe { sys::Cobalt_TexelArray_RemoveOutputCaptureTarget(self.handle, output.handle) }
    }
}

impl Drop for TexelArray {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_TexelArray_Delete(self.handle);
        }
    }
}

unsafe impl Send for TexelArray {}
unsafe impl Sync for TexelArray {}
