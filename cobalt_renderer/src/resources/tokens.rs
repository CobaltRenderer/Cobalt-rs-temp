// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License

/// ID to link a vertex attribute to a shader
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexAttributeId(pub(crate) u32);

/// ID to link a state value to a shader
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateValueId(pub(crate) u32);

/// ID to link a texture to a shader
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureId(pub(crate) u32);

/// ID to link a sampler to a shader
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SamplerId(pub(crate) u32);

/// ID to link a state buffer to a shader
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateBufferId(pub(crate) u32);

/// ID to link a texel or data array to a shader
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceArrayId(pub(crate) u32);
