// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use num_enum::TryFromPrimitive;

#[cfg(feature = "raw_window_handle")]
use raw_window_handle::RawDisplayHandle;

use super::GraphicsDeviceEnumerator;
use crate::renderer::{Renderer, RendererInitializationFlags, RendererOption};
use crate::resources::textures;
use crate::{RendererError, RendererErrorKind, RendererResult};

use cobalt_renderer_sys as sys;

#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceType {
    Discrete = sys::Cobalt_DeviceType_Discrete as i32,
    Integrated = sys::Cobalt_DeviceType_Integrated as i32,
    Software = sys::Cobalt_DeviceType_Software as i32,
    Unknown = sys::Cobalt_DeviceType_Unknown as i32,
}

#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryType {
    Dedicated = sys::Cobalt_MemoryType_Dedicated as i32,
    Shared = sys::Cobalt_MemoryType_Shared as i32,
}

#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    AnisotropicFiltering = sys::Cobalt_Feature_AnisotropicFiltering as i32,
    GeometryShaders = sys::Cobalt_Feature_GeometryShaders as i32,
    ComputeShaders = sys::Cobalt_Feature_ComputeShaders as i32,
    MeshShaders = sys::Cobalt_Feature_MeshShaders as i32,
    DepthBiasClamp = sys::Cobalt_Feature_DepthBiasClamp as i32,
    IndirectDraw = sys::Cobalt_Feature_IndirectDraw as i32,
    IndirectMultiDrawNative = sys::Cobalt_Feature_IndirectMultiDrawNative as i32,
    InstanceOffset = sys::Cobalt_Feature_InstanceOffset as i32,
    PolygonWireframeFillMode = sys::Cobalt_Feature_PolygonWireframeFillMode as i32,
    ResourceArrays = sys::Cobalt_Feature_ResourceArrays as i32,
    ShaderArraysOfArrays = sys::Cobalt_Feature_ShaderArraysOfArrays as i32,
    SeparateBlendModePerTarget = sys::Cobalt_Feature_SeparateBlendModePerTarget as i32,
    SeparateTextureSamplers = sys::Cobalt_Feature_SeparateTextureSamplers as i32,
    TextureCubeArray = sys::Cobalt_Feature_TextureCubeArray as i32,
    MipmapLevelBias = sys::Cobalt_Feature_MipmapLevelBias as i32,
}

#[repr(i32)]
#[derive(TryFromPrimitive, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DepthRange {
    #[default]
    ZeroToOne = sys::Cobalt_DepthRange_ZeroToOne as i32,
    NegativeOneToOne = sys::Cobalt_DepthRange_NegativeOneToOne as i32,
}

/// Manufacturer of graphics device
#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vendor {
    Apple = sys::Cobalt_Vendor_Apple as i32,
    Amd = sys::Cobalt_Vendor_AMD as i32,
    ImgTec = sys::Cobalt_Vendor_ImgTec as i32,
    Nvidia = sys::Cobalt_Vendor_Nvidia as i32,
    Arm = sys::Cobalt_Vendor_ARM as i32,
    Microsoft = sys::Cobalt_Vendor_Microsoft as i32,
    Qualcomm = sys::Cobalt_Vendor_Qualcomm as i32,
    Intel = sys::Cobalt_Vendor_Intel as i32,
    Mesa = sys::Cobalt_Vendor_Mesa as i32,
    Vivante = sys::Cobalt_Vendor_Vivante as i32,
    VeriSilicon = sys::Cobalt_Vendor_VeriSilicon as i32,
    Unknown = sys::Cobalt_Vendor_Unknown as i32,
}

/// Reported image size and dimension limits for graphics device
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ImageLimits {
    pub max_image_dimension_texture_1d: u32,
    pub max_image_dimension_texture_2d: u32,
    pub max_image_dimension_texture_3d: u32,
    pub max_image_dimension_texture_cube: u32,
    pub max_image_array_size_texture_1d: u32,
    pub max_image_array_size_texture_2d: u32,
    pub max_sampler_anisotropic_filtering_level: i32,
}

/// Reported limits in shaders for graphics device
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ShaderLimits {
    pub max_vertex_shader_input_attributes: u32,
    pub max_vertex_shader_output_components: u32,
    pub max_geometry_shader_input_components: u32,
    pub max_geometry_shader_output_components: u32,
    pub max_geometry_shader_output_vertices: u32,
    pub max_geometry_shader_total_output_components: u32,
    pub max_fragment_shader_input_components: u32,
}

/// Reported limits for renderables for graphics device
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct DrawLimits {
    pub max_vertex_count_per_draw: u32,
    pub max_index_value: u32,
    pub max_texture_resources_per_draw: u32,
    pub max_resources_per_draw: u32,
}

/// Reported limits for frame buffers for graphics device
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct FrameBufferLimits {
    pub max_frame_buffer_width: u32,
    pub max_frame_buffer_height: u32,
    pub max_frame_buffer_color_attachments: u32,
    pub depth_range: DepthRange,
}

/// Reported limits for state buffers for graphics device
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct DataBufferLimits {
    pub max_state_buffer_page_size: u32,
    pub state_buffer_alignment_float_or_int: u32,
    pub state_buffer_alignment_vector_4f: u32,
    pub state_buffer_alignment_matrix_4f: u32,
    pub state_buffer_alignment_array_whole: u32,
    pub state_buffer_alignment_array_stride: u32,
    pub state_buffer_alignment_struct: u32,
}

/// Window system that will be used for the renderer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowSystem {
    Headless,
    #[cfg(target_os = "windows")]
    Windows,
    #[cfg(target_os = "macos")]
    AppKit,
    #[cfg(target_os = "linux")]
    Wayland {
        display: std::ptr::NonNull<std::ffi::c_void>,
    },
    #[cfg(target_os = "linux")]
    Xcb {
        connection: std::ptr::NonNull<std::ffi::c_void>,
    },
    #[cfg(target_os = "linux")]
    Xlib {
        display: std::ptr::NonNull<std::ffi::c_void>,
    },
}

impl WindowSystem {
    #[cfg(feature = "raw_window_handle")]
    pub fn new_from_raw_display_handle(
        handle: raw_window_handle::RawDisplayHandle,
    ) -> RendererResult<Self> {
        match handle {
            #[cfg(target_os = "windows")]
            RawDisplayHandle::Windows(_) => Ok(Self::Windows),
            #[cfg(target_os = "macos")]
            RawDisplayHandle::AppKit(_) => Ok(Self::AppKit),
            #[cfg(target_os = "linux")]
            RawDisplayHandle::Wayland(w) => Ok(Self::Wayland { display: w.display }),
            #[cfg(target_os = "linux")]
            RawDisplayHandle::Xcb(w) => {
                let connection = w.connection.ok_or_else(|| {
                    RendererError::new_with_error(
                        RendererErrorKind::UnsupportedWindow,
                        "Xcb display does not contain a required connection pointer".into(),
                    )
                })?;
                Ok(Self::Xcb { connection })
            }
            #[cfg(target_os = "linux")]
            RawDisplayHandle::Xlib(w) => {
                let display = w.display.ok_or_else(|| {
                    RendererError::new_with_error(
                        RendererErrorKind::UnsupportedWindow,
                        "Xlib display does not contain a required display pointer".into(),
                    )
                })?;
                Ok(Self::Xlib { display })
            }
            _ => Err(RendererError::new_with_error(
                RendererErrorKind::UnsupportedWindow,
                "RawDisplayHandle variant is not supported on this platform".into(),
            )),
        }
    }
}

/// Graphics device for rendering, allows for interrogating
/// available features and limits, and creating a [`Renderer`](crate::renderer::Renderer)
pub struct GraphicsDevice<'a> {
    pub(crate) handle: sys::Cobalt_GraphicsDevice,
    enumerator: &'a GraphicsDeviceEnumerator,
}

impl<'a> GraphicsDevice<'a> {
    pub(crate) fn new(
        handle: sys::Cobalt_GraphicsDevice,
        enumerator: &'a GraphicsDeviceEnumerator,
    ) -> Self {
        GraphicsDevice { handle, enumerator }
    }

    pub fn device_type(&self) -> DeviceType {
        let mut device_type = sys::Cobalt_DeviceType_Unknown;
        unsafe {
            sys::Cobalt_GraphicsDevice_GetDeviceType(self.handle, &mut device_type);
        }
        DeviceType::try_from_primitive(device_type as i32).unwrap()
    }

    pub fn vendor(&self) -> Vendor {
        let mut vendor = sys::Cobalt_Vendor_Unknown;
        unsafe {
            sys::Cobalt_GraphicsDevice_GetVendor(self.handle, &mut vendor);
        }
        Vendor::try_from_primitive(vendor as i32).unwrap()
    }

    pub fn memory_size_in_bytes(&self, memory_type: MemoryType) -> usize {
        unsafe {
            sys::Cobalt_GraphicsDevice_GetMemorySizeInBytes(
                self.handle,
                memory_type as sys::Cobalt_MemoryType,
            )
        }
    }

    pub fn vendor_name(&self) -> String {
        // We don't know the size of the string.
        // Allocate what should be enough space and if it fills up
        // we will fetch the name again but with enough space
        let mut capacity: usize = 64;
        loop {
            let mut name: Vec<u8> = vec![0; capacity];
            let mut length = name.len();
            unsafe {
                sys::Cobalt_GraphicsDevice_GetVendorName(
                    self.handle,
                    name.as_mut_ptr() as *mut std::ffi::c_char,
                    &mut length,
                );
            }
            if length > name.len() {
                capacity = length;
                continue;
            }

            name.truncate(length);
            return String::from_utf8_lossy(name.as_slice()).to_string();
        }
    }

    pub fn device_name(&self) -> String {
        // We don't know the size of the string.
        // Allocate what should be enough space and if it fills up
        // we will fetch the name again but with enough space
        let mut capacity: usize = 128;
        loop {
            let mut name: Vec<u8> = vec![0; capacity];
            let mut length = name.len();
            unsafe {
                sys::Cobalt_GraphicsDevice_GetDeviceName(
                    self.handle,
                    name.as_mut_ptr() as *mut std::ffi::c_char,
                    &mut length,
                );
            }
            if length > name.len() {
                capacity = length;
                continue;
            }

            name.truncate(length);
            return String::from_utf8_lossy(name.as_slice()).to_string();
        }
    }

    pub fn driver_info(&self) -> String {
        // We don't know the size of the string.
        // Allocate what should be enough space and if it fills up
        // we will fetch the name again but with enough space
        let mut capacity: usize = 128;
        loop {
            let mut name: Vec<u8> = vec![0; capacity];
            let mut length = name.len();
            unsafe {
                sys::Cobalt_GraphicsDevice_GetDriverInfo(
                    self.handle,
                    name.as_mut_ptr() as *mut std::ffi::c_char,
                    &mut length,
                );
            }
            if length > name.len() {
                capacity = length;
                continue;
            }

            name.truncate(length);
            return String::from_utf8_lossy(name.as_slice()).to_string();
        }
    }

    pub fn image_limits(&self) -> ImageLimits {
        let mut limits = sys::Cobalt_ImageLimits::default();
        unsafe {
            sys::Cobalt_GraphicsDevice_GetImageLimits(self.handle, &mut limits);
        }
        ImageLimits {
            max_image_dimension_texture_1d: limits.maxImageDimensionTexture1D,
            max_image_dimension_texture_2d: limits.maxImageDimensionTexture2D,
            max_image_dimension_texture_3d: limits.maxImageDimensionTexture3D,
            max_image_array_size_texture_1d: limits.maxImageArraySizeTexture1D,
            max_image_array_size_texture_2d: limits.maxImageArraySizeTexture2D,
            max_image_dimension_texture_cube: limits.maxImageDimensionTextureCube,
            max_sampler_anisotropic_filtering_level: limits.maxSamplerAnisotropicFilteringLevel,
        }
    }

    pub fn shader_limits(&self) -> ShaderLimits {
        let mut limits = sys::Cobalt_ShaderLimits::default();
        unsafe {
            sys::Cobalt_GraphicsDevice_GetShaderLimits(self.handle, &mut limits);
        }
        ShaderLimits {
            max_vertex_shader_input_attributes: limits.maxVertexShaderInputAttributes,
            max_vertex_shader_output_components: limits.maxVertexShaderOutputComponents,
            max_geometry_shader_input_components: limits.maxGeometryShaderInputComponents,
            max_geometry_shader_output_components: limits.maxGeometryShaderOutputComponents,
            max_geometry_shader_output_vertices: limits.maxGeometryShaderOutputVertices,
            max_geometry_shader_total_output_components: limits
                .maxGeometryShaderTotalOutputComponents,
            max_fragment_shader_input_components: limits.maxFragmentShaderInputComponents,
        }
    }

    pub fn draw_limits(&self) -> DrawLimits {
        let mut limits = sys::Cobalt_DrawLimits::default();
        unsafe {
            sys::Cobalt_GraphicsDevice_GetDrawLimits(self.handle, &mut limits);
        }
        DrawLimits {
            max_vertex_count_per_draw: limits.maxVertexCountPerDraw,
            max_index_value: limits.maxIndexValue,
            max_texture_resources_per_draw: limits.maxTextureResourcesPerDraw,
            max_resources_per_draw: limits.maxResourcesPerDraw,
        }
    }

    pub fn draw_frame_buffer_limits(&self) -> FrameBufferLimits {
        let mut limits = sys::Cobalt_FrameBufferLimits::default();
        unsafe {
            sys::Cobalt_GraphicsDevice_GetFrameBufferLimits(self.handle, &mut limits);
        }
        FrameBufferLimits {
            max_frame_buffer_width: limits.maxFrameBufferWidth,
            max_frame_buffer_height: limits.maxFrameBufferHeight,
            max_frame_buffer_color_attachments: limits.maxFrameBufferColorAttachments,
            depth_range: DepthRange::try_from_primitive(limits.depthRange as i32).unwrap(),
        }
    }

    pub fn draw_data_buffer_limits(&self) -> DataBufferLimits {
        let mut limits = sys::Cobalt_DataBufferLimits::default();
        unsafe {
            sys::Cobalt_GraphicsDevice_GetDataBufferLimits(self.handle, &mut limits);
        }
        DataBufferLimits {
            max_state_buffer_page_size: limits.maxStateBufferPageSize,
            state_buffer_alignment_float_or_int: limits.stateBufferAlignmentFloatOrInt,
            state_buffer_alignment_vector_4f: limits.stateBufferAlignmentVector4f,
            state_buffer_alignment_matrix_4f: limits.stateBufferAlignmentMatrix4f,
            state_buffer_alignment_array_whole: limits.stateBufferAlignmentArrayWhole,
            state_buffer_alignment_array_stride: limits.stateBufferAlignmentArrayStride,
            state_buffer_alignment_struct: limits.stateBufferAlignmentStruct,
        }
    }

    pub fn is_feature_supported(&self, feature: Feature) -> bool {
        unsafe {
            sys::Cobalt_GraphicsDevice_IsFeatureSupported(
                self.handle,
                feature as sys::Cobalt_Feature,
            ) != 0
        }
    }

    pub fn are_all_features_supported(&self, feature_set: &[Feature]) -> bool {
        unsafe {
            sys::Cobalt_GraphicsDevice_AreAllFeaturesSupported(
                self.handle,
                feature_set.as_ptr() as *const sys::Cobalt_Feature,
                feature_set.len(),
            ) != 0
        }
    }

    pub fn all_supported_features(&self) -> Vec<Feature> {
        // In theory, we know the maximum number of features available
        // as long as the `Feature` enum stays in sync, we could even use
        // std::mem::variant_count (https://doc.rust-lang.org/std/mem/fn.variant_count.html)
        // But for now we will allocate space and resize if needed
        // like we do for `device_name`
        let mut capacity = 16;
        loop {
            let mut features: Vec<sys::Cobalt_Feature> = vec![0; capacity];
            let mut length = features.len();
            unsafe {
                sys::Cobalt_GraphicsDevice_GetAllSupportedFeatures(
                    self.handle,
                    features.as_mut_ptr(),
                    &mut length,
                );
            }
            if length > features.len() {
                capacity = length;
                continue;
            }

            features.truncate(length);
            let features: Result<Vec<Feature>, num_enum::TryFromPrimitiveError<Feature>> = features
                .into_iter()
                .map(|f| Feature::try_from_primitive(f as i32))
                .collect();
            return features.unwrap();
        }
    }

    pub fn is_texture_format_supported(
        &self,
        image_format: textures::ImageFormat,
        data_format: textures::DataFormat,
    ) -> bool {
        unsafe {
            sys::Cobalt_GraphicsDevice_IsTextureFormatSupported(
                self.handle,
                image_format as sys::Cobalt_ImageFormat,
                data_format as sys::Cobalt_DataFormat,
            ) != 0
        }
    }

    pub fn create_renderer(
        &mut self,
        enabled_features: &[Feature],
        enabled_options: &[RendererOption],
        init_flags: RendererInitializationFlags,
        window_system: WindowSystem,
    ) -> RendererResult<Renderer> {
        let mut renderer = std::ptr::null_mut();
        unsafe {
            return_on_failure!(sys::Cobalt_GraphicsDevice_CreateRenderer(
                self.handle,
                enabled_features.as_ptr() as *const sys::Cobalt_Feature,
                enabled_features.len(),
                enabled_options.as_ptr() as *const sys::Cobalt_RendererOption,
                enabled_options.len(),
                &mut renderer,
            ));
            match window_system {
                WindowSystem::Headless => {
                    let info = sys::Cobalt_WindowSystemInfoHeadless {
                        base: sys::Cobalt_WindowSystemInfoBase {
                            type_: sys::Cobalt_WindowSystemType_Headless,
                        },
                    };
                    return_on_failure!(sys::Cobalt_Renderer_Initialize(
                        renderer,
                        (&raw const info) as *const sys::Cobalt_WindowSystemInfoBase,
                        init_flags.bits() as sys::Cobalt_RendererInitializationFlags
                    ))
                }
                #[cfg(target_os = "windows")]
                WindowSystem::Windows => {
                    let info = sys::Cobalt_WindowSystemInfoWin32 {
                        base: sys::Cobalt_WindowSystemInfoBase {
                            type_: sys::Cobalt_WindowSystemType_Win32,
                        },
                    };
                    return_on_failure!(sys::Cobalt_Renderer_Initialize(
                        renderer,
                        (&raw const info) as *const sys::Cobalt_WindowSystemInfoBase,
                        init_flags.bits() as sys::Cobalt_RendererInitializationFlags
                    ))
                }
                #[cfg(target_os = "macos")]
                WindowSystem::AppKit => {
                    let info = sys::Cobalt_WindowSystemInfoAppKit {
                        base: sys::Cobalt_WindowSystemInfoBase {
                            type_: sys::Cobalt_WindowSystemType_AppKit,
                        },
                    };
                    return_on_failure!(sys::Cobalt_Renderer_Initialize(
                        renderer,
                        (&raw const info) as *const sys::Cobalt_WindowSystemInfoBase,
                        init_flags.bits() as sys::Cobalt_RendererInitializationFlags
                    ))
                }
                #[cfg(target_os = "linux")]
                WindowSystem::Wayland { display } => {
                    let info = sys::Cobalt_WindowSystemInfoWayland {
                        base: sys::Cobalt_WindowSystemInfoBase {
                            type_: sys::Cobalt_WindowSystemType_Wayland,
                        },
                        display: display.as_ptr(),
                    };
                    return_on_failure!(sys::Cobalt_Renderer_Initialize(
                        renderer,
                        (&raw const info) as *const sys::Cobalt_WindowSystemInfoBase,
                        init_flags.bits() as sys::Cobalt_RendererInitializationFlags
                    ))
                }
                #[cfg(target_os = "linux")]
                WindowSystem::Xcb { connection } => {
                    let info = sys::Cobalt_WindowSystemInfoXCB {
                        base: sys::Cobalt_WindowSystemInfoBase {
                            type_: sys::Cobalt_WindowSystemType_XCB,
                        },
                        connection: connection.as_ptr(),
                    };
                    return_on_failure!(sys::Cobalt_Renderer_Initialize(
                        renderer,
                        (&raw const info) as *const sys::Cobalt_WindowSystemInfoBase,
                        init_flags.bits() as sys::Cobalt_RendererInitializationFlags
                    ))
                }
                #[cfg(target_os = "linux")]
                WindowSystem::Xlib { display } => {
                    let info = sys::Cobalt_WindowSystemInfoXlib {
                        base: sys::Cobalt_WindowSystemInfoBase {
                            type_: sys::Cobalt_WindowSystemType_Xlib,
                        },
                        display: display.as_ptr(),
                    };
                    return_on_failure!(sys::Cobalt_Renderer_Initialize(
                        renderer,
                        (&raw const info) as *const sys::Cobalt_WindowSystemInfoBase,
                        init_flags.bits() as sys::Cobalt_RendererInitializationFlags
                    ))
                }
            }
        }

        Ok(Renderer::new(renderer, self.enumerator.plugin.clone()))
    }
}

unsafe impl<'a> Send for GraphicsDevice<'a> {}
unsafe impl<'a> Sync for GraphicsDevice<'a> {}
