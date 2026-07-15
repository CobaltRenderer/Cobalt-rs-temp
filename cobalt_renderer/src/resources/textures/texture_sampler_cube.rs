// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use std::sync::Arc;

use super::{FilterMode, MipmapMode, TextureSampler, WrapMode};
use crate::{render_tree::StateContainer, renderer::RendererInternal, resources::SamplerId};

use cobalt_renderer_sys as sys;

pub struct TextureSamplerCube {
    pub(crate) handle: sys::Cobalt_TextureSamplerCube,
    _renderer: Arc<RendererInternal>,
}

impl TextureSamplerCube {
    pub(crate) fn new(
        handle: sys::Cobalt_TextureSamplerCube,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        TextureSamplerCube {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn set_texture_wrap_mode(
        &mut self,
        wrap_mode_horizontal: WrapMode,
        wrap_mode_vertical: WrapMode,
    ) {
        unsafe {
            sys::Cobalt_TextureSamplerCube_SetTextureWrapMode(
                self.handle,
                wrap_mode_horizontal as sys::Cobalt_WrapMode,
                wrap_mode_vertical as sys::Cobalt_WrapMode,
            )
        }
    }

    pub fn set_texture_filter_mode(
        &mut self,
        filter_mode_shrink: FilterMode,
        filter_mode_expand: FilterMode,
    ) {
        unsafe {
            sys::Cobalt_TextureSamplerCube_SetTextureFilterMode(
                self.handle,
                filter_mode_shrink as sys::Cobalt_FilterMode,
                filter_mode_expand as sys::Cobalt_FilterMode,
            )
        }
    }

    pub fn set_texture_mipmap_mode(&mut self, mipmap_mode: MipmapMode) {
        unsafe {
            sys::Cobalt_TextureSamplerCube_SetTextureMipmapMode(
                self.handle,
                mipmap_mode as sys::Cobalt_MipmapMode,
            )
        }
    }

    pub fn set_texture_level_mapping(&mut self, min_level: f32, max_level: f32, level_bias: f32) {
        unsafe {
            sys::Cobalt_TextureSamplerCube_SetMipmapLevelMapping(
                self.handle,
                min_level,
                max_level,
                level_bias,
            )
        }
    }
}

impl TextureSampler for TextureSamplerCube {
    fn bind_to_state_container(
        &mut self,
        sampler_id: SamplerId,
        container: &mut impl StateContainer,
    ) {
        unsafe {
            sys::Cobalt_StateContainer_BindSamplerCube(
                container.node_handle(),
                sampler_id.0,
                self.handle,
            )
        }
    }
}

impl Drop for TextureSamplerCube {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_TextureSamplerCube_Delete(self.handle);
        }
    }
}

unsafe impl Send for TextureSamplerCube {}
unsafe impl Sync for TextureSamplerCube {}
