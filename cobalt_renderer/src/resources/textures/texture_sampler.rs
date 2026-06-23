// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License

use crate::{render_tree::StateContainer, resources::SamplerId};

/// How textures are sampled outside their bounds
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapMode {
    ClampToEdge,
    Repeat,
    RepeatMirrored,
}

/// Filtering mode when sampling textures
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    Nearest,
    Linear,
}

/// Filtering mode when generating mipmaps
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MipmapMode {
    None,
    Nearest,
    Linear,
}

// Rust definitions

/// Defines how to sample a texture
pub trait TextureSampler {
    #[doc(hidden)]
    fn bind_to_state_container(&self, sampler_id: SamplerId, container: &mut impl StateContainer);
}
