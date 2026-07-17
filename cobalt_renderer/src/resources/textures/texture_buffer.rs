// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use bitflags::bitflags;
use num_enum::TryFromPrimitive;

use crate::{render_tree::StateContainer, resources::TextureId};

use cobalt_renderer_sys as sys;

#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    R = sys::Cobalt_ImageFormat_R as i32,
    RG = sys::Cobalt_ImageFormat_RG as i32,
    RGB = sys::Cobalt_ImageFormat_RGB as i32,
    RGBA = sys::Cobalt_ImageFormat_RGBA as i32,
    BGR = sys::Cobalt_ImageFormat_BGR as i32,
    BGRA = sys::Cobalt_ImageFormat_BGRA as i32,
    X = sys::Cobalt_ImageFormat_X as i32,
    XY = sys::Cobalt_ImageFormat_XY as i32,
    XYZ = sys::Cobalt_ImageFormat_XYZ as i32,
    XYZW = sys::Cobalt_ImageFormat_XYZW as i32,
    Depth = sys::Cobalt_ImageFormat_Depth as i32,
    DepthAndStencil = sys::Cobalt_ImageFormat_DepthAndStencil as i32,
}

#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceImageFormat {
    R = sys::Cobalt_SourceImageFormat_R as i32,
    RG = sys::Cobalt_SourceImageFormat_RG as i32,
    RGB = sys::Cobalt_SourceImageFormat_RGB as i32,
    RGBA = sys::Cobalt_SourceImageFormat_RGBA as i32,
    BGR = sys::Cobalt_SourceImageFormat_BGR as i32,
    BGRA = sys::Cobalt_SourceImageFormat_BGRA as i32,
    X = sys::Cobalt_SourceImageFormat_X as i32,
    XY = sys::Cobalt_SourceImageFormat_XY as i32,
    XYZ = sys::Cobalt_SourceImageFormat_XYZ as i32,
    XYZW = sys::Cobalt_SourceImageFormat_XYZW as i32,
}

#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    Int8 = sys::Cobalt_DataFormat_Int8 as i32,
    Int16 = sys::Cobalt_DataFormat_Int16 as i32,
    Int32 = sys::Cobalt_DataFormat_Int32 as i32,
    UInt8 = sys::Cobalt_DataFormat_UInt8 as i32,
    UInt16 = sys::Cobalt_DataFormat_UInt16 as i32,
    UInt32 = sys::Cobalt_DataFormat_UInt32 as i32,
    Norm8 = sys::Cobalt_DataFormat_Norm8 as i32,
    Norm16 = sys::Cobalt_DataFormat_Norm16 as i32,
    UNorm8 = sys::Cobalt_DataFormat_UNorm8 as i32,
    UNorm16 = sys::Cobalt_DataFormat_UNorm16 as i32,
    Float16 = sys::Cobalt_DataFormat_Float16 as i32,
    Float32 = sys::Cobalt_DataFormat_Float32 as i32,
    DXT1 = sys::Cobalt_DataFormat_DXT1 as i32,
    DXT3 = sys::Cobalt_DataFormat_DXT3 as i32,
    DXT5 = sys::Cobalt_DataFormat_DXT5 as i32,
    ETC2 = sys::Cobalt_DataFormat_ETC2 as i32,
    BPTC = sys::Cobalt_DataFormat_BPTC as i32,
    ASTC4x4 = sys::Cobalt_DataFormat_ASTC4x4 as i32,
    ASTC5x5 = sys::Cobalt_DataFormat_ASTC5x5 as i32,
    ASTC6x6 = sys::Cobalt_DataFormat_ASTC6x6 as i32,
    ASTC8x8 = sys::Cobalt_DataFormat_ASTC8x8 as i32,
    DepthUNorm16 = sys::Cobalt_DataFormat_DepthUNorm16 as i32,
    DepthUNorm24 = sys::Cobalt_DataFormat_DepthUNorm24 as i32,
    DepthUNorm24StencilUInt8 = sys::Cobalt_DataFormat_DepthUNorm24StencilUInt8 as i32,
    DepthFloat32 = sys::Cobalt_DataFormat_DepthFloat32 as i32,
    DepthFloat32StencilUInt8 = sys::Cobalt_DataFormat_DepthFloat32StencilUInt8 as i32,
}

#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceDataFormat {
    Int8 = sys::Cobalt_SourceDataFormat_Int8 as i32,
    Int16 = sys::Cobalt_SourceDataFormat_Int16 as i32,
    Int32 = sys::Cobalt_SourceDataFormat_Int32 as i32,
    UInt8 = sys::Cobalt_SourceDataFormat_UInt8 as i32,
    UInt16 = sys::Cobalt_SourceDataFormat_UInt16 as i32,
    UInt32 = sys::Cobalt_SourceDataFormat_UInt32 as i32,
    Norm8 = sys::Cobalt_SourceDataFormat_Norm8 as i32,
    Norm16 = sys::Cobalt_SourceDataFormat_Norm16 as i32,
    Norm32 = sys::Cobalt_SourceDataFormat_Norm32 as i32,
    UNorm8 = sys::Cobalt_SourceDataFormat_UNorm8 as i32,
    UNorm16 = sys::Cobalt_SourceDataFormat_UNorm16 as i32,
    UNorm32 = sys::Cobalt_SourceDataFormat_UNorm32 as i32,
    Float16 = sys::Cobalt_SourceDataFormat_Float16 as i32,
    Float32 = sys::Cobalt_SourceDataFormat_Float32 as i32,
    DXT1 = sys::Cobalt_SourceDataFormat_DXT1 as i32,
    DXT3 = sys::Cobalt_SourceDataFormat_DXT3 as i32,
    DXT5 = sys::Cobalt_SourceDataFormat_DXT5 as i32,
    ETC2 = sys::Cobalt_SourceDataFormat_ETC2 as i32,
    BPTC = sys::Cobalt_SourceDataFormat_BPTC as i32,
    ASTC4x4 = sys::Cobalt_SourceDataFormat_ASTC4x4 as i32,
    ASTC5x5 = sys::Cobalt_SourceDataFormat_ASTC5x5 as i32,
    ASTC6x6 = sys::Cobalt_SourceDataFormat_ASTC6x6 as i32,
    ASTC8x8 = sys::Cobalt_SourceDataFormat_ASTC8x8 as i32,
}

pub const fn equivalent_image_formats(
    source_format: SourceImageFormat,
    format: ImageFormat,
) -> bool {
    match source_format {
        SourceImageFormat::R | SourceImageFormat::X => {
            matches!(format, ImageFormat::R | ImageFormat::X)
        }
        SourceImageFormat::RG | SourceImageFormat::XY => {
            matches!(format, ImageFormat::RG | ImageFormat::XY)
        }
        SourceImageFormat::RGB | SourceImageFormat::XYZ => {
            matches!(format, ImageFormat::RGB | ImageFormat::XYZ)
        }
        SourceImageFormat::RGBA | SourceImageFormat::XYZW => {
            matches!(format, ImageFormat::RGBA | ImageFormat::XYZW)
        }
        SourceImageFormat::BGR => matches!(format, ImageFormat::BGR),
        SourceImageFormat::BGRA => matches!(format, ImageFormat::BGRA),
    }
}

impl SourceImageFormat {
    pub const fn element_count_per_pixel(&self) -> usize {
        match self {
            Self::R | Self::X => 1,
            Self::RG | Self::XY => 2,
            Self::RGB | Self::BGR | Self::XYZ => 3,
            Self::RGBA | Self::BGRA | Self::XYZW => 4,
        }
    }
}

impl ImageFormat {
    pub const fn element_count_per_pixel(&self) -> usize {
        match self {
            Self::R | Self::X | Self::Depth | Self::DepthAndStencil => 1,
            Self::RG | Self::XY => 2,
            Self::RGB | Self::BGR | Self::XYZ => 3,
            Self::RGBA | Self::BGRA | Self::XYZW => 4,
        }
    }
}

pub const fn equivalent_data_formats(source_format: SourceDataFormat, format: DataFormat) -> bool {
    match source_format {
        SourceDataFormat::Int8 => matches!(format, DataFormat::Int8),
        SourceDataFormat::Int16 => matches!(format, DataFormat::Int16),
        SourceDataFormat::Int32 => matches!(format, DataFormat::Int32),
        SourceDataFormat::UNorm8 | SourceDataFormat::UInt8 => {
            matches!(format, DataFormat::UInt8 | DataFormat::UNorm8)
        }
        SourceDataFormat::UNorm16 | SourceDataFormat::UInt16 => matches!(
            format,
            DataFormat::UInt16 | DataFormat::UNorm16 | DataFormat::DepthUNorm16
        ),
        SourceDataFormat::UInt32 => matches!(format, DataFormat::UInt32),
        SourceDataFormat::Norm8 => matches!(format, DataFormat::Norm8),
        SourceDataFormat::Norm16 => matches!(format, DataFormat::Norm16),
        SourceDataFormat::Float16 => matches!(format, DataFormat::Float16),
        SourceDataFormat::Float32 => {
            matches!(format, DataFormat::Float32 | DataFormat::DepthFloat32)
        }
        SourceDataFormat::DXT1 => matches!(format, DataFormat::DXT1),
        SourceDataFormat::DXT3 => matches!(format, DataFormat::DXT3),
        SourceDataFormat::DXT5 => matches!(format, DataFormat::DXT5),
        SourceDataFormat::ETC2 => matches!(format, DataFormat::ETC2),
        SourceDataFormat::BPTC => matches!(format, DataFormat::BPTC),
        SourceDataFormat::ASTC4x4 => matches!(format, DataFormat::ASTC4x4),
        SourceDataFormat::ASTC5x5 => matches!(format, DataFormat::ASTC5x5),
        SourceDataFormat::ASTC6x6 => matches!(format, DataFormat::ASTC6x6),
        SourceDataFormat::ASTC8x8 => matches!(format, DataFormat::ASTC8x8),
        _ => false,
    }
}

pub const fn cell_size_in_bytes_for_format(
    image_format: ImageFormat,
    data_format: DataFormat,
) -> usize {
    match data_format {
        DataFormat::DXT1 => 8,
        DataFormat::DXT3
        | DataFormat::DXT5
        | DataFormat::BPTC
        | DataFormat::ETC2
        | DataFormat::ASTC4x4
        | DataFormat::ASTC5x5
        | DataFormat::ASTC6x6
        | DataFormat::ASTC8x8 => 16,
        f => image_format.element_count_per_pixel() * f.bytes_per_element(),
    }
}

pub const fn cell_size_in_bytes_for_source_format(
    image_format: SourceImageFormat,
    data_format: SourceDataFormat,
) -> usize {
    match data_format {
        SourceDataFormat::DXT1 => 8,
        SourceDataFormat::DXT3
        | SourceDataFormat::DXT5
        | SourceDataFormat::BPTC
        | SourceDataFormat::ETC2
        | SourceDataFormat::ASTC4x4
        | SourceDataFormat::ASTC5x5
        | SourceDataFormat::ASTC6x6
        | SourceDataFormat::ASTC8x8 => 16,
        f => image_format.element_count_per_pixel() * f.bytes_per_element(),
    }
}

impl DataFormat {
    pub const fn bytes_per_element(&self) -> usize {
        match self {
            Self::Int8 | Self::UInt8 | Self::Norm8 | Self::UNorm8 => 1,
            Self::Int16 | Self::UInt16 | Self::Norm16 | Self::UNorm16 | Self::Float16 => 2,
            Self::Int32 | Self::UInt32 | Self::Float32 => 4,
            Self::DepthUNorm16 => 2,
            Self::DepthUNorm24 | Self::DepthUNorm24StencilUInt8 | Self::DepthFloat32 => 4,
            Self::DepthFloat32StencilUInt8 => 8,
            _ => 0,
        }
    }

    pub const fn is_compressed_format(&self) -> bool {
        matches!(
            self,
            Self::DXT1
                | Self::DXT3
                | Self::DXT5
                | Self::ETC2
                | Self::BPTC
                | Self::ASTC4x4
                | Self::ASTC5x5
                | Self::ASTC6x6
                | Self::ASTC8x8
        )
    }

    pub const fn cell_dimensions_in_pixels(&self) -> [u32; 2] {
        match self {
            Self::DXT1 | Self::DXT3 | Self::DXT5 | Self::ETC2 | Self::BPTC | Self::ASTC4x4 => {
                [4, 4]
            }
            Self::ASTC5x5 => [5, 5],
            Self::ASTC6x6 => [6, 6],
            Self::ASTC8x8 => [8, 8],
            _ => [1, 1],
        }
    }
}

impl SourceDataFormat {
    pub const fn bytes_per_element(&self) -> usize {
        match self {
            Self::Int8 | Self::UInt8 | Self::Norm8 | Self::UNorm8 => 1,
            Self::Int16 | Self::UInt16 | Self::Norm16 | Self::UNorm16 | Self::Float16 => 2,
            Self::Int32 | Self::UInt32 | Self::Norm32 | Self::UNorm32 | Self::Float32 => 4,
            _ => 0,
        }
    }

    pub const fn is_compressed_format(&self) -> bool {
        matches!(
            self,
            Self::DXT1
                | Self::DXT3
                | Self::DXT5
                | Self::ETC2
                | Self::BPTC
                | Self::ASTC4x4
                | Self::ASTC5x5
                | Self::ASTC6x6
                | Self::ASTC8x8
        )
    }

    pub const fn cell_dimensions_in_pixels(&self) -> [u32; 2] {
        match self {
            Self::DXT1 | Self::DXT3 | Self::DXT5 | Self::ETC2 | Self::BPTC | Self::ASTC4x4 => {
                [4, 4]
            }
            Self::ASTC5x5 => [5, 5],
            Self::ASTC6x6 => [6, 6],
            Self::ASTC8x8 => [8, 8],
            _ => [1, 1],
        }
    }

    pub const fn cell_dimensions_in_bytes(&self) -> [u32; 2] {
        match self {
            Self::DXT1 | Self::DXT3 | Self::DXT5 | Self::ETC2 => [16, 16],
            _ => [4, 4],
        }
    }
}

#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeMapFace {
    PositiveX = sys::Cobalt_CubeMapFace_PositiveX as i32,
    NegativeX = sys::Cobalt_CubeMapFace_NegativeX as i32,
    PositiveY = sys::Cobalt_CubeMapFace_PositiveY as i32,
    NegativeY = sys::Cobalt_CubeMapFace_NegativeY as i32,
    PositiveZ = sys::Cobalt_CubeMapFace_PositiveZ as i32,
    NegativeZ = sys::Cobalt_CubeMapFace_NegativeZ as i32,
}

#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleCount {
    SampleCount1 = sys::Cobalt_SampleCount_1 as i32,
    SampleCount2 = sys::Cobalt_SampleCount_2 as i32,
    SampleCount4 = sys::Cobalt_SampleCount_4 as i32,
    SampleCount8 = sys::Cobalt_SampleCount_8 as i32,
    SampleCount16 = sys::Cobalt_SampleCount_16 as i32,
    SampleCount32 = sys::Cobalt_SampleCount_32 as i32,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TextureUsageFlags : u32
    {
        const Default = sys::Cobalt_TextureUsageFlags_Default as u32;
        const ShaderInput = sys::Cobalt_TextureUsageFlags_ShaderInput as u32;
        const FrameBufferOutput = sys::Cobalt_TextureUsageFlags_FrameBufferOutput as u32;
        const MultiSampleResolve = sys::Cobalt_TextureUsageFlags_MultiSampleResolve as u32;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PerformanceHint : u32
    {
        const Default = sys::Cobalt_TexturePerformanceHint_Default as u32;
        const ReadNever = sys::Cobalt_TexturePerformanceHint_ReadNever as u32;
        const ReadRarely = sys::Cobalt_TexturePerformanceHint_ReadRarely as u32;
        const ReadOften = sys::Cobalt_TexturePerformanceHint_ReadOften as u32;
        const ReadFlagsMask = sys::Cobalt_TexturePerformanceHint_ReadFlagsMask as u32;
        const WriteNever = sys::Cobalt_TexturePerformanceHint_WriteNever as u32;
        const WriteRarely = sys::Cobalt_TexturePerformanceHint_WriteRarely as u32;
        const WriteOften = sys::Cobalt_TexturePerformanceHint_WriteOften as u32;
        const WriteFlagsMask = sys::Cobalt_TexturePerformanceHint_WriteFlagsMask as u32;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DataPersistenceFlags : u32 {
        const PersistAlways = sys::Cobalt_TextureDataPersistenceFlags_PersistAlways as u32;
        const InvalidateExistingDataOnWrite = sys::Cobalt_TextureDataPersistenceFlags_InvalidateExistingDataOnWrite as u32;
        const InvalidateExistingDataAfterDrawComplete = sys::Cobalt_TextureDataPersistenceFlags_InvalidateExistingDataAfterDrawComplete as u32;
    }
}

// This is a workaround for Rust not having generic specialization
// which would allow us to have different functions under the same name
// that would take different input types and have different implementations.
// Instead we have a trait which we only implement on some types
// which then have specializations. Then we can have one generic
// function which takes this trait and calls the specialized function
// on the type. Not ideal but functional

pub trait TextureBuffer {
    #[doc(hidden)]
    fn texture_handle(&self) -> sys::Cobalt_TextureBuffer;

    fn set_usage_flags(&mut self, usage_flags: TextureUsageFlags) {
        unsafe {
            sys::Cobalt_TextureBuffer_SetUsageFlags(
                self.texture_handle(),
                usage_flags.bits() as sys::Cobalt_TextureUsageFlags,
            );
        }
    }

    fn set_performance_hints(
        &mut self,
        performance_hint_cpu: PerformanceHint,
        performance_hint_gpu: PerformanceHint,
    ) {
        unsafe {
            sys::Cobalt_TextureBuffer_SetPerformanceHints(
                self.texture_handle(),
                performance_hint_cpu.bits() as sys::Cobalt_TexturePerformanceHint,
                performance_hint_gpu.bits() as sys::Cobalt_TexturePerformanceHint,
            );
        }
    }

    fn set_data_persistence_flags(&mut self, data_persistence_flags: DataPersistenceFlags) {
        unsafe {
            sys::Cobalt_TextureBuffer_SetDataPersistenceFlags(
                self.texture_handle(),
                data_persistence_flags.bits() as sys::Cobalt_TextureDataPersistenceFlags,
            );
        }
    }

    #[doc(hidden)]
    fn bind_to_state_container(
        &mut self,
        texture_id: TextureId,
        container: &mut impl StateContainer,
    );
}
