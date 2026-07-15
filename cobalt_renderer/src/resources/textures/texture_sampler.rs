// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

use crate::{render_tree::StateContainer, resources::SamplerId};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapMode {
    ClampToEdge,
    Repeat,
    RepeatMirrored,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    Nearest,
    Linear,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MipmapMode {
    None,
    Nearest,
    Linear,
}

// This is a workaround for Rust not having generic specialization
// which would allow us to have different functions under the same name
// that would take different input types and have different implementations.
// Instead we have a trait which we only implement on some types
// which then have specializations. Then we can have one generic
// function which takes this trait and calls the specialized function
// on the type. Not ideal but functional

pub trait TextureSampler {
    #[doc(hidden)]
    fn bind_to_state_container(
        &mut self,
        sampler_id: SamplerId,
        container: &mut impl StateContainer,
    );
}
