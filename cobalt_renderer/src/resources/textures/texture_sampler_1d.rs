// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License
use std::sync::Arc;

use super::{FilterMode, MipmapMode, TextureSampler, WrapMode};
use crate::{render_tree::StateContainer, renderer::RendererInternal, resources::SamplerId};

use cobalt_renderer_sys as sys;

/// Sampler for [`TextureBuffer1D`](`super::TextureBuffer1D`)
pub struct TextureSampler1D {
    pub(crate) handle: sys::Cobalt_TextureSampler1D,
    _renderer: Arc<RendererInternal>,
}

impl TextureSampler1D {
    pub(crate) fn new(
        handle: sys::Cobalt_TextureSampler1D,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        TextureSampler1D {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn set_texture_wrap_mode(&mut self, wrap_mode_horizontal: WrapMode) {
        unsafe {
            sys::Cobalt_TextureSampler1D_SetTextureWrapMode(
                self.handle,
                wrap_mode_horizontal as sys::Cobalt_WrapMode,
            )
        }
    }

    pub fn set_texture_filter_mode(
        &mut self,
        filter_mode_shrink: FilterMode,
        filter_mode_expand: FilterMode,
    ) {
        unsafe {
            sys::Cobalt_TextureSampler1D_SetTextureFilterMode(
                self.handle,
                filter_mode_shrink as sys::Cobalt_FilterMode,
                filter_mode_expand as sys::Cobalt_FilterMode,
            )
        }
    }

    pub fn set_texture_mipmap_mode(&mut self, mipmap_mode: MipmapMode) {
        unsafe {
            sys::Cobalt_TextureSampler1D_SetTextureMipmapMode(
                self.handle,
                mipmap_mode as sys::Cobalt_MipmapMode,
            )
        }
    }

    pub fn set_texture_level_mapping(&mut self, min_level: f32, max_level: f32, level_bias: f32) {
        unsafe {
            sys::Cobalt_TextureSampler1D_SetMipmapLevelMapping(
                self.handle,
                min_level,
                max_level,
                level_bias,
            )
        }
    }
}

impl TextureSampler for TextureSampler1D {
    fn bind_to_state_container(&self, sampler_id: SamplerId, container: &mut impl StateContainer) {
        unsafe {
            sys::Cobalt_StateContainer_BindSampler1D(
                container.node_handle(),
                sampler_id.0,
                self.handle,
            )
        }
    }
}

impl Drop for TextureSampler1D {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_TextureSampler1D_Delete(self.handle);
        }
    }
}

unsafe impl Send for TextureSampler1D {}
unsafe impl Sync for TextureSampler1D {}
