// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

use std::sync::Arc;

use crate::{RendererResult, renderer::RendererInternal};

use cobalt_renderer_sys as sys;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartTiming {
    AfterCurrentFrame = sys::Cobalt_StartTiming_AfterCurrentFrame as i32,
    Immediately = sys::Cobalt_StartTiming_Immediately as i32,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndTiming {
    BeforeNextFrame = sys::Cobalt_EndTiming_BeforeNextFrame as i32,
    AnyFrame = sys::Cobalt_EndTiming_AnyFrame as i32,
}

pub struct TransferBatch {
    pub(crate) handle: sys::Cobalt_TransferBatch,
    _renderer: Arc<RendererInternal>,
}

impl TransferBatch {
    pub(crate) fn new(
        handle: sys::Cobalt_TransferBatch,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        TransferBatch {
            handle,
            _renderer: renderer_internal,
        }
    }

    // NOTE(DTM): Bug in 2.0.0, SubmitBatch returns u8 instead of u32,
    // cast should be removed in future versions when this is fixed and
    // a warning is raised
    #[warn(clippy::unnecessary_cast)]
    pub fn submit_batch(&self) -> RendererResult<()> {
        unsafe {
            let result = sys::Cobalt_TransferBatch_SubmitBatch(self.handle) as i32;
            return_on_failure!(result);
        }
        Ok(())
    }

    pub fn is_submitted(&self) -> bool {
        unsafe { sys::Cobalt_TransferBatch_IsSubmitted(self.handle) != 0 }
    }

    pub fn is_complete(&self) -> bool {
        unsafe { sys::Cobalt_TransferBatch_IsComplete(self.handle) != 0 }
    }

    pub fn wait_for_complete(&self) {
        unsafe { sys::Cobalt_TransferBatch_WaitForComplete(self.handle) }
    }
}

impl Drop for TransferBatch {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_TransferBatch_Delete(self.handle);
        }
    }
}

unsafe impl Send for TransferBatch {}
unsafe impl Sync for TransferBatch {}
