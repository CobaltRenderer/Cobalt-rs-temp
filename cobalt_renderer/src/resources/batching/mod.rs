// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License

//! Utilities for batching data transfers and controlling timing

use std::sync::Arc;

use crate::renderer::RendererInternal;

use cobalt_renderer_sys as sys;

/// When the transfer can begin after being submitted
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartTiming {
    AfterCurrentFrame = sys::Cobalt_StartTiming_AfterCurrentFrame as i32,
    Immediately = sys::Cobalt_StartTiming_Immediately as i32,
}

/// When the transfer must be complete by
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndTiming {
    BeforeNextFrame = sys::Cobalt_EndTiming_BeforeNextFrame as i32,
    AnyFrame = sys::Cobalt_EndTiming_AnyFrame as i32,
}

/// Batches together one or more data transfers, allowing for specified start and end times
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

    pub fn submit_batch(&self) {
        unsafe {
            sys::Cobalt_TransferBatch_SubmitBatch(self.handle);
        }
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
