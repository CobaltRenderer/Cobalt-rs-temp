// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use std::sync::Arc;

use super::StateContainer;
use crate::renderer::RendererInternal;

use cobalt_renderer_sys as sys;

pub struct DefaultState {
    pub(crate) handle: sys::Cobalt_DefaultState,
    _renderer: Arc<RendererInternal>,
}

impl DefaultState {
    pub(crate) fn new(
        handle: sys::Cobalt_DefaultState,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        DefaultState {
            handle,
            _renderer: renderer_internal,
        }
    }
}

impl StateContainer for DefaultState {
    fn node_handle(&mut self) -> sys::Cobalt_StateContainer {
        self.handle as sys::Cobalt_StateContainer
    }
}

impl Drop for DefaultState {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_DefaultState_Delete(self.handle);
        }
    }
}

unsafe impl Send for DefaultState {}
unsafe impl Sync for DefaultState {}
