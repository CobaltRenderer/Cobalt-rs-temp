// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use bitflags::bitflags;
use num_enum::TryFromPrimitive;
use std::sync::Arc;

use crate::RendererResult;
use crate::renderer::RendererInternal;
use crate::resources::data::StateBufferLayout;
use crate::resources::{
    ResourceArrayId, SamplerId, StateBufferId, StateValueId, TextureId, VertexAttributeId,
};

use cobalt_renderer_sys as sys;

#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStage {
    Vertex = sys::Cobalt_ShaderStage_Vertex as i32,
    Fragment = sys::Cobalt_ShaderStage_Fragment as i32,
    Geometry = sys::Cobalt_ShaderStage_Geometry as i32,
    Compute = sys::Cobalt_ShaderStage_Compute as i32,
}

#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeFormat {
    Hlsl = sys::Cobalt_CodeFormat_HLSL as i32,
    Dxbc = sys::Cobalt_CodeFormat_DXBC as i32,
    Dxil = sys::Cobalt_CodeFormat_DXIL as i32,
    Spirv = sys::Cobalt_CodeFormat_SPIRV as i32,
    SpirvAssembly = sys::Cobalt_CodeFormat_SPIRVAssembly as i32,
    Glsl = sys::Cobalt_CodeFormat_GLSL as i32,
    Msl = sys::Cobalt_CodeFormat_MSL as i32,
    Air = sys::Cobalt_CodeFormat_AIR as i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderSourceInfo<'a> {
    Hlsl {
        code: &'a str,
        entry_point_function_name: Option<&'a str>,
    },
    Dxbc {
        code: &'a [u8],
    },
    Dxil {
        code: &'a [u8],
    },
    SpirvAssembly {
        code: &'a str,
        entry_point_function_name: Option<&'a str>,
    },
    Spirv {
        code: &'a [u32],
        entry_point_function_name: Option<&'a str>,
    },
    Glsl {
        code: &'a str,
    },
    Msl {
        code: &'a str,
        entry_point_function_name: Option<&'a str>,
    },
    Air {
        code: &'a [u8],
        entry_point_function_name: Option<&'a str>,
    },
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ShaderTargetInfoOpenglFlags : i32 {
        const None = sys::Cobalt_ShaderTargetInfoOpenGL_Flags_None as i32;
        const ForceGlsl = sys::Cobalt_ShaderTargetInfoOpenGL_Flags_ForceGLSL as i32;
        const ForceSpirvIfAvailable = sys::Cobalt_ShaderTargetInfoOpenGL_Flags_ForceSPIRVIfAvailable as i32;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ShaderTargetInfoDirect3dFlags : i32 {
        const None = sys::Cobalt_ShaderTargetInfoDirect3D_Flags_None as i32;
        const ForceFXC = sys::Cobalt_ShaderTargetInfoDirect3D_Flags_ForceFXC as i32;
        const EnableDebugInfo = sys::Cobalt_ShaderTargetInfoDirect3D_Flags_EnableDebugInfo as i32;
        const SkipOptimization = sys::Cobalt_ShaderTargetInfoDirect3D_Flags_SkipOptimization as i32;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderTargetInfo {
    Direct3d {
        flags: ShaderTargetInfoDirect3dFlags,
        target_shader_model_major: u32,
        target_shader_model_minor: u32,
    },
    OpenGl {
        flags: ShaderTargetInfoOpenglFlags,
    },
    Vulkan,
    Metal,
}

pub struct ShaderProgram {
    pub(crate) handle: sys::Cobalt_ShaderProgram,
    _renderer: Arc<RendererInternal>,
}

impl ShaderProgram {
    pub(crate) fn new(
        handle: sys::Cobalt_ShaderProgram,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        ShaderProgram {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn is_code_format_supported(&self, format: CodeFormat) -> bool {
        unsafe {
            sys::Cobalt_ShaderProgram_IsCodeFormatSupported(
                self.handle,
                format as sys::Cobalt_CodeFormat,
            ) != 0
        }
    }

    pub fn preferred_code_format(&self) -> CodeFormat {
        let value = unsafe { sys::Cobalt_ShaderProgram_PreferredCodeFormat(self.handle) };
        CodeFormat::try_from_primitive(value as i32).unwrap()
    }

    pub fn configure_shader_target(&mut self, target_info: ShaderTargetInfo) -> RendererResult<()> {
        match target_info {
            ShaderTargetInfo::Direct3d {
                flags,
                target_shader_model_major,
                target_shader_model_minor,
            } => {
                let target = sys::Cobalt_ShaderTargetInfoDirect3D {
                    base: sys::Cobalt_ShaderTargetInfoBase {
                        type_: sys::Cobalt_ShaderTargetInfoType_Direct3D,
                    },
                    flags: flags.bits() as sys::Cobalt_ShaderTargetInfoDirect3D_Flags,
                    targetShaderModelMajor: target_shader_model_major,
                    targetShaderModelMinor: target_shader_model_minor,
                };
                unsafe {
                    return_on_failure!(sys::Cobalt_ShaderProgram_ConfigureShaderTarget(
                        self.handle,
                        &raw const target as *const sys::Cobalt_ShaderTargetInfoBase
                    ))
                }
            }
            ShaderTargetInfo::OpenGl { flags } => {
                let target = sys::Cobalt_ShaderTargetInfoOpenGL {
                    base: sys::Cobalt_ShaderTargetInfoBase {
                        type_: sys::Cobalt_ShaderTargetInfoType_OpenGL,
                    },
                    flags: flags.bits() as sys::Cobalt_ShaderTargetInfoOpenGL_Flags,
                };
                unsafe {
                    return_on_failure!(sys::Cobalt_ShaderProgram_ConfigureShaderTarget(
                        self.handle,
                        &raw const target as *const sys::Cobalt_ShaderTargetInfoBase
                    ))
                }
            }
            ShaderTargetInfo::Metal => {
                let target = sys::Cobalt_ShaderTargetInfoBase {
                    type_: sys::Cobalt_ShaderTargetInfoType_Metal,
                };
                unsafe {
                    return_on_failure!(sys::Cobalt_ShaderProgram_ConfigureShaderTarget(
                        self.handle,
                        &raw const target
                    ))
                }
            }
            ShaderTargetInfo::Vulkan => {
                let target = sys::Cobalt_ShaderTargetInfoBase {
                    type_: sys::Cobalt_ShaderTargetInfoType_Vulkan,
                };
                unsafe {
                    return_on_failure!(sys::Cobalt_ShaderProgram_ConfigureShaderTarget(
                        self.handle,
                        &raw const target
                    ))
                }
            }
        }
        Ok(())
    }

    pub fn load_shader_stage(
        &mut self,
        stage: ShaderStage,
        source_info: ShaderSourceInfo,
    ) -> RendererResult<()> {
        match source_info {
            ShaderSourceInfo::Hlsl {
                code,
                entry_point_function_name,
            } => {
                let (name, name_len) = match entry_point_function_name {
                    None => (std::ptr::null(), 0),
                    Some(n) => (n.as_ptr(), n.len()),
                };
                let info = sys::Cobalt_ShaderSourceInfoHLSL {
                    base: sys::Cobalt_ShaderSourceInfoBase {
                        type_: sys::Cobalt_CodeFormat_HLSL,
                    },
                    code: code.as_ptr() as *const std::ffi::c_char,
                    codeSizeInBytes: code.len(),
                    entryPointFunctionName: name as *const std::ffi::c_char,
                    entryPointFunctionNameSizeInBytes: name_len,
                };
                unsafe {
                    return_on_failure!(sys::Cobalt_ShaderProgram_LoadShaderStage(
                        self.handle,
                        stage as sys::Cobalt_ShaderStage,
                        (&raw const info) as *const sys::Cobalt_ShaderSourceInfoBase,
                    ))
                }
            }
            ShaderSourceInfo::Air {
                code,
                entry_point_function_name,
            } => {
                let (name, name_len) = match entry_point_function_name {
                    None => (std::ptr::null(), 0),
                    Some(n) => (n.as_ptr(), n.len()),
                };
                let info = sys::Cobalt_ShaderSourceInfoAIR {
                    base: sys::Cobalt_ShaderSourceInfoBase {
                        type_: sys::Cobalt_CodeFormat_AIR,
                    },
                    code: code.as_ptr(),
                    codeSizeInBytes: code.len(),
                    entryPointFunctionName: name as *const std::ffi::c_char,
                    entryPointFunctionNameSizeInBytes: name_len,
                };
                unsafe {
                    return_on_failure!(sys::Cobalt_ShaderProgram_LoadShaderStage(
                        self.handle,
                        stage as sys::Cobalt_ShaderStage,
                        (&raw const info) as *const sys::Cobalt_ShaderSourceInfoBase,
                    ))
                }
            }
            ShaderSourceInfo::Dxbc { code } => {
                let info = sys::Cobalt_ShaderSourceInfoDXBC {
                    base: sys::Cobalt_ShaderSourceInfoBase {
                        type_: sys::Cobalt_CodeFormat_DXBC,
                    },
                    code: code.as_ptr(),
                    codeSizeInBytes: code.len(),
                };
                unsafe {
                    return_on_failure!(sys::Cobalt_ShaderProgram_LoadShaderStage(
                        self.handle,
                        stage as sys::Cobalt_ShaderStage,
                        (&raw const info) as *const sys::Cobalt_ShaderSourceInfoBase,
                    ))
                }
            }
            ShaderSourceInfo::Dxil { code } => {
                let info = sys::Cobalt_ShaderSourceInfoDXIL {
                    base: sys::Cobalt_ShaderSourceInfoBase {
                        type_: sys::Cobalt_CodeFormat_DXIL,
                    },
                    code: code.as_ptr(),
                    codeSizeInBytes: code.len(),
                };
                unsafe {
                    return_on_failure!(sys::Cobalt_ShaderProgram_LoadShaderStage(
                        self.handle,
                        stage as sys::Cobalt_ShaderStage,
                        (&raw const info) as *const sys::Cobalt_ShaderSourceInfoBase,
                    ))
                }
            }
            ShaderSourceInfo::Glsl { code } => {
                let info = sys::Cobalt_ShaderSourceInfoGLSL {
                    base: sys::Cobalt_ShaderSourceInfoBase {
                        type_: sys::Cobalt_CodeFormat_GLSL,
                    },
                    code: code.as_ptr() as *const std::ffi::c_char,
                    codeSizeInBytes: code.len(),
                };
                unsafe {
                    return_on_failure!(sys::Cobalt_ShaderProgram_LoadShaderStage(
                        self.handle,
                        stage as sys::Cobalt_ShaderStage,
                        (&raw const info) as *const sys::Cobalt_ShaderSourceInfoBase,
                    ))
                }
            }
            ShaderSourceInfo::Msl {
                code,
                entry_point_function_name,
            } => {
                let (name, name_len) = match entry_point_function_name {
                    None => (std::ptr::null(), 0),
                    Some(n) => (n.as_ptr(), n.len()),
                };
                let info = sys::Cobalt_ShaderSourceInfoMSL {
                    base: sys::Cobalt_ShaderSourceInfoBase {
                        type_: sys::Cobalt_CodeFormat_MSL,
                    },
                    code: code.as_ptr() as *const std::ffi::c_char,
                    codeSizeInBytes: code.len(),
                    entryPointFunctionName: name as *const std::ffi::c_char,
                    entryPointFunctionNameSizeInBytes: name_len,
                };
                unsafe {
                    return_on_failure!(sys::Cobalt_ShaderProgram_LoadShaderStage(
                        self.handle,
                        stage as sys::Cobalt_ShaderStage,
                        (&raw const info) as *const sys::Cobalt_ShaderSourceInfoBase,
                    ))
                }
            }
            ShaderSourceInfo::Spirv {
                code,
                entry_point_function_name,
            } => {
                let (name, name_len) = match entry_point_function_name {
                    None => (std::ptr::null(), 0),
                    Some(n) => (n.as_ptr(), n.len()),
                };
                let info = sys::Cobalt_ShaderSourceInfoSPIRV {
                    base: sys::Cobalt_ShaderSourceInfoBase {
                        type_: sys::Cobalt_CodeFormat_SPIRV,
                    },
                    code: code.as_ptr(),
                    codeSizeInUnits: code.len(),
                    entryPointFunctionName: name as *const std::ffi::c_char,
                    entryPointFunctionNameSizeInBytes: name_len,
                };
                unsafe {
                    return_on_failure!(sys::Cobalt_ShaderProgram_LoadShaderStage(
                        self.handle,
                        stage as sys::Cobalt_ShaderStage,
                        (&raw const info) as *const sys::Cobalt_ShaderSourceInfoBase,
                    ))
                }
            }
            ShaderSourceInfo::SpirvAssembly {
                code,
                entry_point_function_name,
            } => {
                let (name, name_len) = match entry_point_function_name {
                    None => (std::ptr::null(), 0),
                    Some(n) => (n.as_ptr(), n.len()),
                };
                let info = sys::Cobalt_ShaderSourceInfoSPIRVAssembly {
                    base: sys::Cobalt_ShaderSourceInfoBase {
                        type_: sys::Cobalt_CodeFormat_SPIRVAssembly,
                    },
                    code: code.as_ptr() as *const std::ffi::c_char,
                    codeSizeInBytes: code.len(),
                    entryPointFunctionName: name as *const std::ffi::c_char,
                    entryPointFunctionNameSizeInBytes: name_len,
                };
                unsafe {
                    return_on_failure!(sys::Cobalt_ShaderProgram_LoadShaderStage(
                        self.handle,
                        stage as sys::Cobalt_ShaderStage,
                        (&raw const info) as *const sys::Cobalt_ShaderSourceInfoBase,
                    ))
                }
            }
        };

        Ok(())
    }

    pub fn compile_program(&mut self) -> RendererResult<()> {
        unsafe { return_on_failure!(sys::Cobalt_ShaderProgram_CompileProgram(self.handle)) }
        Ok(())
    }

    pub fn vertex_attribute_exists(&self, name: impl AsRef<str>) -> bool {
        let name = name.as_ref();
        unsafe {
            sys::Cobalt_ShaderProgram_VertexAttributeExists(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
            ) != 0
        }
    }

    pub fn state_value_exists(&self, name: impl AsRef<str>) -> bool {
        let name = name.as_ref();
        unsafe {
            sys::Cobalt_ShaderProgram_StateValueExists(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
            ) != 0
        }
    }

    pub fn texture_exists(&self, name: impl AsRef<str>) -> bool {
        let name = name.as_ref();
        unsafe {
            sys::Cobalt_ShaderProgram_TextureExists(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
            ) != 0
        }
    }

    pub fn sampler_exists(&self, name: impl AsRef<str>) -> bool {
        let name = name.as_ref();
        unsafe {
            sys::Cobalt_ShaderProgram_SamplerExists(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
            ) != 0
        }
    }

    pub fn state_buffer_exists(&self, name: impl AsRef<str>) -> bool {
        let name = name.as_ref();
        unsafe {
            sys::Cobalt_ShaderProgram_StateBufferExists(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
            ) != 0
        }
    }

    pub fn resource_array_exists(&self, name: impl AsRef<str>) -> bool {
        let name = name.as_ref();
        unsafe {
            sys::Cobalt_ShaderProgram_ResourceArrayExists(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
            ) != 0
        }
    }

    pub fn vertex_attribute_id(&self, name: impl AsRef<str>) -> Option<VertexAttributeId> {
        let name = name.as_ref();
        unsafe {
            match sys::Cobalt_ShaderProgram_GetVertexAttributeId(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
            ) {
                0xFFFFFFFF => None,
                n => Some(VertexAttributeId(n)),
            }
        }
    }

    pub fn state_value_id(&self, name: impl AsRef<str>) -> Option<StateValueId> {
        let name = name.as_ref();
        unsafe {
            match sys::Cobalt_ShaderProgram_GetStateValueId(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
            ) {
                0xFFFFFFFF => None,
                n => Some(StateValueId(n)),
            }
        }
    }

    pub fn texture_id(&self, name: impl AsRef<str>) -> Option<TextureId> {
        let name = name.as_ref();
        unsafe {
            match sys::Cobalt_ShaderProgram_GetTextureId(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
            ) {
                0xFFFFFFFF => None,
                n => Some(TextureId(n)),
            }
        }
    }

    pub fn sampler_id(&self, name: impl AsRef<str>) -> Option<SamplerId> {
        let name = name.as_ref();
        unsafe {
            match sys::Cobalt_ShaderProgram_GetSamplerId(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
            ) {
                0xFFFFFFFF => None,
                n => Some(SamplerId(n)),
            }
        }
    }

    pub fn state_buffer_id(&self, name: impl AsRef<str>) -> Option<StateBufferId> {
        let name = name.as_ref();
        unsafe {
            match sys::Cobalt_ShaderProgram_GetStateBufferId(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
            ) {
                0xFFFFFFFF => None,
                n => Some(StateBufferId(n)),
            }
        }
    }

    pub fn resource_array_id(&self, name: impl AsRef<str>) -> Option<ResourceArrayId> {
        let name = name.as_ref();
        unsafe {
            match sys::Cobalt_ShaderProgram_GetResourceArrayId(
                self.handle,
                name.as_ptr() as *const std::ffi::c_char,
                name.len(),
            ) {
                0xFFFFFFFF => None,
                n => Some(ResourceArrayId(n)),
            }
        }
    }

    pub fn load_state_buffer_layout_from_shader(
        &self,
        state_buffer_id: StateBufferId,
        state_buffer_layout: &mut StateBufferLayout,
    ) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_ShaderProgram_LoadStateBufferLayoutFromShader(
                self.handle,
                state_buffer_id.0,
                state_buffer_layout.handle,
            ))
        }
        Ok(())
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_ShaderProgram_Delete(self.handle);
        }
    }
}

unsafe impl Send for ShaderProgram {}
unsafe impl Sync for ShaderProgram {}
