// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use std::sync::Arc;

use super::{DefaultState, ProgramNode};
use crate::renderer::RendererInternal;
use crate::resources::frame_buffers::{AttachmentType, FrameBuffer};

use cobalt_renderer_sys as sys;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachmentLoadBehavior {
    LoadExistingData = sys::Cobalt_AttachmentLoadBehavior_LoadExistingData as i32,
    UndefinedInitialData = sys::Cobalt_AttachmentLoadBehavior_UndefinedInitialData as i32,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachmentStoreBehavior {
    StoreFinalData = sys::Cobalt_AttachmentStoreBehavior_StoreFinalData as i32,
    UndefinedFinalData = sys::Cobalt_AttachmentStoreBehavior_UndefinedFinalData as i32,
}

pub struct RenderPassNode {
    pub(crate) handle: sys::Cobalt_RenderPassNode,
    _renderer: Arc<RendererInternal>,
}

// This is a workaround for Rust not having generic specialization
// which would allow us to have different functions under the same name
// that would take different input types and have different implementations.
// Instead we have a trait which we only implement on some types
// which then have specializations. Then we can have one generic
// function which takes this trait and calls the specialized function
// on the type. Not ideal but functional

pub trait ClearDataType: Sized {
    #[doc(hidden)]
    fn set_attachment_clear_data(
        render_pass: &mut RenderPassNode,
        attachment_type: AttachmentType,
        index: usize,
        data: &[Self; 4],
    );
}

impl ClearDataType for f32 {
    fn set_attachment_clear_data(
        render_pass: &mut RenderPassNode,
        attachment_type: AttachmentType,
        index: usize,
        data: &[Self; 4],
    ) {
        unsafe {
            sys::Cobalt_RenderPassNode_SetAttachmentClearDataF(
                render_pass.handle,
                attachment_type as sys::Cobalt_AttachmentType,
                index,
                data,
            )
        }
    }
}

impl ClearDataType for i32 {
    fn set_attachment_clear_data(
        render_pass: &mut RenderPassNode,
        attachment_type: AttachmentType,
        index: usize,
        data: &[Self; 4],
    ) {
        unsafe {
            sys::Cobalt_RenderPassNode_SetAttachmentClearDataI(
                render_pass.handle,
                attachment_type as sys::Cobalt_AttachmentType,
                index,
                data,
            )
        }
    }
}

impl ClearDataType for u32 {
    fn set_attachment_clear_data(
        render_pass: &mut RenderPassNode,
        attachment_type: AttachmentType,
        index: usize,
        data: &[Self; 4],
    ) {
        unsafe {
            sys::Cobalt_RenderPassNode_SetAttachmentClearDataU(
                render_pass.handle,
                attachment_type as sys::Cobalt_AttachmentType,
                index,
                data,
            )
        }
    }
}

impl RenderPassNode {
    pub(crate) fn new(
        handle: sys::Cobalt_RenderPassNode,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        RenderPassNode {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn add_child_node(&self, child_node: &ProgramNode, default_state: Option<&DefaultState>) {
        unsafe {
            match default_state {
                Some(s) => {
                    sys::Cobalt_RenderPassNode_AddChildNode(
                        self.handle,
                        child_node.handle,
                        s.handle,
                    );
                }
                None => {
                    sys::Cobalt_RenderPassNode_AddChildNode(
                        self.handle,
                        child_node.handle,
                        std::ptr::null_mut(),
                    );
                }
            }
        }
    }

    pub fn add_child_nodes(
        &self,
        child_nodes: &[&ProgramNode],
        default_states: Option<&[&DefaultState]>,
    ) {
        unsafe {
            let nodes: Vec<sys::Cobalt_ProgramNode> =
                child_nodes.iter().map(|n| n.handle).collect();
            match default_states {
                Some(s) => {
                    let states: Vec<sys::Cobalt_DefaultState> =
                        s.iter().map(|x| x.handle).collect();
                    sys::Cobalt_RenderPassNode_AddChildNodes(
                        self.handle,
                        nodes.as_ptr(),
                        nodes.len(),
                        states.as_ptr(),
                    );
                }
                None => {
                    sys::Cobalt_RenderPassNode_AddChildNodes(
                        self.handle,
                        nodes.as_ptr(),
                        nodes.len(),
                        std::ptr::null(),
                    );
                }
            }
        }
    }

    pub fn remove_child_node(&self, child_node: &ProgramNode) {
        unsafe {
            sys::Cobalt_RenderPassNode_RemoveChildNode(self.handle, child_node.handle);
        }
    }

    pub fn remove_child_nodes(&self, child_nodes: &[&ProgramNode]) {
        unsafe {
            let nodes: Vec<sys::Cobalt_ProgramNode> =
                child_nodes.iter().map(|n| n.handle).collect();
            sys::Cobalt_RenderPassNode_RemoveChildNodes(self.handle, nodes.as_ptr(), nodes.len());
        }
    }

    pub fn remove_all_child_nodes(&self) {
        unsafe {
            sys::Cobalt_RenderPassNode_RemoveAllChildNodes(self.handle);
        }
    }

    pub fn set_child_nodes(
        &self,
        child_nodes: &[&ProgramNode],
        default_states: Option<&[&DefaultState]>,
    ) {
        unsafe {
            let nodes: Vec<sys::Cobalt_ProgramNode> =
                child_nodes.iter().map(|n| n.handle).collect();
            match default_states {
                Some(s) => {
                    let states: Vec<sys::Cobalt_DefaultState> =
                        s.iter().map(|x| x.handle).collect();
                    sys::Cobalt_RenderPassNode_SetChildNodes(
                        self.handle,
                        nodes.as_ptr(),
                        nodes.len(),
                        states.as_ptr(),
                    );
                }
                None => {
                    sys::Cobalt_RenderPassNode_SetChildNodes(
                        self.handle,
                        nodes.as_ptr(),
                        nodes.len(),
                        std::ptr::null(),
                    );
                }
            }
        }
    }

    pub fn bind_frame_buffer(&mut self, frame_buffer: &FrameBuffer) {
        unsafe {
            sys::Cobalt_RenderPassNode_BindFrameBuffer(self.handle, frame_buffer.handle);
        }
    }

    pub fn set_attachment_clear_data<T>(
        &mut self,
        attachment_type: AttachmentType,
        index: usize,
        data: &[T; 4],
    ) where
        T: ClearDataType,
    {
        T::set_attachment_clear_data(self, attachment_type, index, data)
    }

    pub fn set_attachment_load_store_behavior(
        &mut self,
        attachment_type: AttachmentType,
        index: usize,
        load_behavior: AttachmentLoadBehavior,
        store_behavior: AttachmentStoreBehavior,
    ) {
        unsafe {
            sys::Cobalt_RenderPassNode_SetAttachmentLoadStoreBehavior(
                self.handle,
                attachment_type as sys::Cobalt_AttachmentType,
                index,
                load_behavior as sys::Cobalt_AttachmentLoadBehavior,
                store_behavior as sys::Cobalt_AttachmentStoreBehavior,
            );
        }
    }

    pub fn remove_attachment_clear_data(&mut self, attachment_type: AttachmentType, index: usize) {
        unsafe {
            sys::Cobalt_RenderPassNode_RemoveAttachmentClearData(
                self.handle,
                attachment_type as sys::Cobalt_AttachmentType,
                index,
            );
        }
    }

    pub fn enable_attachment_multi_sampling_resolution(
        &mut self,
        attachment_type: AttachmentType,
        index: usize,
        resolve_attachment_index: Option<usize>,
    ) {
        unsafe {
            sys::Cobalt_RenderPassNode_EnableAttachmentMultiSamplingResolution(
                self.handle,
                attachment_type as sys::Cobalt_AttachmentType,
                index,
                resolve_attachment_index.unwrap_or(usize::MAX),
            );
        }
    }

    pub fn disable_attachment_multi_sampling_resolution(
        &mut self,
        attachment_type: AttachmentType,
        index: usize,
    ) {
        unsafe {
            sys::Cobalt_RenderPassNode_DisableAttachmentMultiSamplingResolution(
                self.handle,
                attachment_type as sys::Cobalt_AttachmentType,
                index,
            );
        }
    }

    pub fn set_is_enabled(&mut self, enabled: bool) {
        unsafe {
            sys::Cobalt_RenderPassNode_SetIsEnabled(self.handle, if enabled { 1 } else { 0 });
        }
    }

    pub fn set_debug_name(&mut self, name: impl AsRef<str>) {
        let bytes = name.as_ref().as_bytes();
        unsafe {
            sys::Cobalt_RenderPassNode_SetDebugName(
                self.handle,
                bytes.as_ptr() as *const std::ffi::c_char,
                bytes.len(),
            );
        }
    }
}

impl Drop for RenderPassNode {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_RenderPassNode_Delete(self.handle);
        }
    }
}

unsafe impl Send for RenderPassNode {}
unsafe impl Sync for RenderPassNode {}
