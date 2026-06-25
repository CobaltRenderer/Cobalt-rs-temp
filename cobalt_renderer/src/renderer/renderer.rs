// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use bitflags::bitflags;
use num_enum::TryFromPrimitive;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;

use crate::RendererPlugin;
use crate::RendererPluginInternal;
use crate::render_tree::*;
use crate::resources::*;

use cobalt_renderer_sys as sys;

/// Optional renderer features
#[repr(i32)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RendererOption {
    EnableDebugLogging = sys::Cobalt_RendererOption_EnableDebugLogging as i32,
    EnableRenderMarkers = sys::Cobalt_RendererOption_EnableRenderMarkers as i32,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RendererInitializationFlags : i32 {
        const None = sys::Cobalt_RendererInitializationFlags_None as i32;
    }
}

/// A shared lock to indicate when graphics work is occurring and prevent the next frame advancing.
///
/// The Cobalt Renderer can work across multiple threads, but the only requirement is no methods can be called
/// on renderer objects when the frame is advancing during [`Renderer::start_new_frame`].
///
/// Methods on graphics objects do not automatically lock the renderer as the caller is typically doing
/// larger operations that should appear atomic and not be interrupted midway through. This requires
/// the caller to manually lock the renderer using this object when they call any graphics methods.
///
/// This object can be cloned and shared. The same underlying lock is shared with the renderer
/// and all clones.
#[derive(Clone)]
pub struct GraphicsLock(Arc<RwLock<()>>);

impl GraphicsLock {
    /// Lock the renderer and permit calling methods on graphics objects. This method creates a
    /// guard which will prevent advancing a frame until it is dropped. If a frame is advancing, this
    /// function will block until it is done.
    ///
    /// It is advised to only hold the guard for a short time as it will block a new frame. If applicable,
    /// consider dropping the guard or re-locking to allow a new frame to begin.
    ///
    /// This function is safe to call recursively.
    pub fn lock(&self) -> RwLockReadGuard<'_, ()> {
        self.0.read_recursive()
    }

    fn frame_lock(&self) -> RwLockWriteGuard<'_, ()> {
        self.0.write()
    }
}

/// Core object for creating objects and managing frames
///
/// `start_new_frame` is used to render a new frame.
/// It's critical that all other graphics work is halted while that function call occurs.
/// An associated [`GraphicsLock`] achieves this.
///
/// All content is added in under render passes which are set with `set_render_passes`.
pub struct Renderer {
    internal: Arc<RendererInternal>,
    graphics_lock: GraphicsLock,
}

/// RendererInternal actually holds the handle to the C++ Renderer object
/// It's expected that this type is wrapped in an `std::sync::Arc`
pub(crate) struct RendererInternal {
    pub(crate) handle: sys::Cobalt_Renderer,
    plugin: Arc<RendererPluginInternal>,
}

impl Renderer {
    pub(crate) fn new(handle: sys::Cobalt_Renderer, plugin: Arc<RendererPluginInternal>) -> Self {
        let lock = GraphicsLock(Arc::new(RwLock::new(())));
        Renderer {
            internal: Arc::new(RendererInternal { handle, plugin }),
            graphics_lock: lock,
        }
    }

    pub(crate) fn handle(&self) -> sys::Cobalt_Renderer {
        self.internal.handle
    }

    pub(crate) fn internal_clone(&self) -> Arc<RendererInternal> {
        self.internal.clone()
    }

    pub fn create_vertex_attribute(
        &self,
        data_type: geometry::VertexAttributeType,
        element_count: usize,
        vertex_count: usize,
        performance_hint_cpu: geometry::PerformanceHint,
        performance_hint_gpu: geometry::PerformanceHint,
        data_persistence_flags: geometry::DataPersistenceFlags,
    ) -> geometry::VertexAttribute {
        geometry::VertexAttribute::new(
            self,
            data_type,
            element_count,
            vertex_count,
            performance_hint_cpu,
            performance_hint_gpu,
            data_persistence_flags,
        )
    }

    pub fn create_index_attribute(
        &self,
        data_type: geometry::IndexAttributeType,
        index_count: usize,
        performance_hint_cpu: geometry::PerformanceHint,
        performance_hint_gpu: geometry::PerformanceHint,
        data_persistence_flags: geometry::DataPersistenceFlags,
    ) -> geometry::IndexAttribute {
        geometry::IndexAttribute::new(
            self,
            data_type,
            index_count,
            performance_hint_cpu,
            performance_hint_gpu,
            data_persistence_flags,
        )
    }

    pub fn create_vertex_buffer(&self) -> geometry::VertexBuffer {
        let mut vertex_buffer = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateVertexBuffer(self.internal.handle, &mut vertex_buffer);
        }
        geometry::VertexBuffer::new(vertex_buffer, self.internal.clone())
    }

    pub fn create_index_buffer(&self) -> geometry::IndexBuffer {
        let mut index_buffer = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateIndexBuffer(self.internal.handle, &mut index_buffer);
        }
        geometry::IndexBuffer::new(index_buffer, self.internal.clone())
    }

    pub fn create_texel_array(&self) -> data::TexelArray {
        let mut array = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTexelArray(self.internal.handle, &mut array);
        }
        data::TexelArray::new(array, self.internal.clone())
    }

    pub fn create_texel_array_output(&self) -> data::TexelArrayOutput {
        let mut array = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTexelArrayOutput(self.internal.handle, &mut array);
        }
        data::TexelArrayOutput::new(array, self.internal.clone())
    }

    pub fn create_data_array(&self) -> data::DataArray {
        let mut array = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateDataArray(self.internal.handle, &mut array);
        }
        data::DataArray::new(array, self.internal.clone())
    }

    pub fn create_data_array_output(&self) -> data::DataArrayOutput {
        let mut array = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateDataArrayOutput(self.internal.handle, &mut array);
        }
        data::DataArrayOutput::new(array, self.internal.clone())
    }

    pub fn create_texture_buffer_1d(&self) -> textures::TextureBuffer1D {
        let mut texture_buffer = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTextureBuffer1D(self.internal.handle, &mut texture_buffer);
        }
        textures::TextureBuffer1D::new(texture_buffer, self.internal.clone())
    }

    pub fn create_texture_buffer_2d(&self) -> textures::TextureBuffer2D {
        let mut texture_buffer = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTextureBuffer2D(self.internal.handle, &mut texture_buffer);
        }
        textures::TextureBuffer2D::new(texture_buffer, self.internal.clone())
    }

    pub fn create_texture_buffer_3d(&self) -> textures::TextureBuffer3D {
        let mut texture_buffer = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTextureBuffer3D(self.internal.handle, &mut texture_buffer);
        }
        textures::TextureBuffer3D::new(texture_buffer, self.internal.clone())
    }

    pub fn create_texture_buffer_cube(&self) -> textures::TextureBufferCube {
        let mut texture_buffer = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTextureBufferCube(self.internal.handle, &mut texture_buffer);
        }
        textures::TextureBufferCube::new(texture_buffer, self.internal.clone())
    }

    pub fn create_texture_buffer_1d_array(&self) -> textures::TextureBuffer1DArray {
        let mut texture_buffer = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTextureBuffer1DArray(
                self.internal.handle,
                &mut texture_buffer,
            );
        }
        textures::TextureBuffer1DArray::new(texture_buffer, self.internal.clone())
    }

    pub fn create_texture_buffer_2d_array(&self) -> textures::TextureBuffer2DArray {
        let mut texture_buffer = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTextureBuffer2DArray(
                self.internal.handle,
                &mut texture_buffer,
            );
        }
        textures::TextureBuffer2DArray::new(texture_buffer, self.internal.clone())
    }

    pub fn create_texture_buffer_cube_array(&self) -> textures::TextureBufferCubeArray {
        let mut texture_buffer = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTextureBufferCubeArray(
                self.internal.handle,
                &mut texture_buffer,
            );
        }
        textures::TextureBufferCubeArray::new(texture_buffer, self.internal.clone())
    }

    pub fn create_texture_sampler_1d(&self) -> textures::TextureSampler1D {
        let mut texture_sampler = std::ptr::null_mut();

        unsafe {
            sys::Cobalt_Renderer_CreateTextureSampler1D(self.internal.handle, &mut texture_sampler);
        }
        textures::TextureSampler1D::new(texture_sampler, self.internal.clone())
    }

    pub fn create_texture_sampler_2d(&self) -> textures::TextureSampler2D {
        let mut texture_sampler = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTextureSampler2D(self.internal.handle, &mut texture_sampler);
        }
        textures::TextureSampler2D::new(texture_sampler, self.internal.clone())
    }

    pub fn create_texture_sampler_3d(&self) -> textures::TextureSampler3D {
        let mut texture_sampler = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTextureSampler3D(self.internal.handle, &mut texture_sampler);
        }
        textures::TextureSampler3D::new(texture_sampler, self.internal.clone())
    }

    pub fn create_texture_sampler_cube(&self) -> textures::TextureSamplerCube {
        let mut texture_sampler = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTextureSamplerCube(
                self.internal.handle,
                &mut texture_sampler,
            );
        }
        textures::TextureSamplerCube::new(texture_sampler, self.internal.clone())
    }

    pub fn create_texture_sampler_1d_array(&self) -> textures::TextureSampler1DArray {
        let mut texture_sampler = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTextureSampler1DArray(
                self.internal.handle,
                &mut texture_sampler,
            );
        }
        textures::TextureSampler1DArray::new(texture_sampler, self.internal.clone())
    }

    pub fn create_texture_sampler_2d_array(&self) -> textures::TextureSampler2DArray {
        let mut texture_sampler = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTextureSampler2DArray(
                self.internal.handle,
                &mut texture_sampler,
            );
        }
        textures::TextureSampler2DArray::new(texture_sampler, self.internal.clone())
    }

    pub fn create_texture_sampler_cube_array(&self) -> textures::TextureSamplerCubeArray {
        let mut texture_sampler = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTextureSamplerCubeArray(
                self.internal.handle,
                &mut texture_sampler,
            );
        }
        textures::TextureSamplerCubeArray::new(texture_sampler, self.internal.clone())
    }

    pub fn create_transfer_batch(
        &self,
        start_timing: batching::StartTiming,
        end_timing: batching::EndTiming,
    ) -> batching::TransferBatch {
        let mut batch = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateTransferBatch(
                self.internal.handle,
                &mut batch,
                start_timing as sys::Cobalt_StartTiming,
                end_timing as sys::Cobalt_EndTiming,
            );
        }
        batching::TransferBatch::new(batch, self.internal.clone())
    }

    pub fn create_frame_buffer(&self) -> frame_buffers::FrameBuffer {
        let mut frame_buffer = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateFrameBuffer(self.internal.handle, &mut frame_buffer);
        }
        frame_buffers::FrameBuffer::new(frame_buffer, self.internal.clone())
    }

    pub fn create_frame_buffer_output(&self) -> frame_buffers::FrameBufferOutput {
        let mut frame_buffer_output = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateFrameBufferOutput(
                self.internal.handle,
                &mut frame_buffer_output,
            );
        }
        frame_buffers::FrameBufferOutput::new(frame_buffer_output, self.internal.clone())
    }

    pub fn create_state_buffer(&self) -> data::StateBuffer {
        let mut state_buffer = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateStateBuffer(self.internal.handle, &mut state_buffer);
        }
        data::StateBuffer::new(state_buffer, self.internal.clone())
    }

    pub fn create_state_buffer_layout(&self) -> data::StateBufferLayout {
        let mut state_buffer_layout = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateStateBufferLayout(
                self.internal.handle,
                &mut state_buffer_layout,
            );
        }
        data::StateBufferLayout::new(state_buffer_layout, self.internal.clone())
    }

    pub fn create_render_pass_node(&self) -> RenderPassNode {
        let mut render_pass_node = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateRenderPassNode(self.internal.handle, &mut render_pass_node);
        }
        RenderPassNode::new(render_pass_node, self.internal.clone())
    }

    pub fn create_program_node(&self) -> ProgramNode {
        let mut program_node = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateProgramNode(self.internal.handle, &mut program_node);
        }
        ProgramNode::new(program_node, self.internal.clone())
    }

    pub fn create_state_group_node(&self) -> StateGroupNode {
        let mut state_group_node = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateStateGroupNode(self.internal.handle, &mut state_group_node);
        }
        StateGroupNode::new(state_group_node, self.internal.clone())
    }

    pub fn create_renderable_node(&self) -> RenderableNode {
        let mut renderable_node = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateRenderableNode(self.internal.handle, &mut renderable_node);
        }
        RenderableNode::new(renderable_node, self.internal.clone())
    }

    pub fn create_default_state(&self) -> DefaultState {
        let mut default_state = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateDefaultState(self.internal.handle, &mut default_state);
        }
        DefaultState::new(default_state, self.internal.clone())
    }

    pub fn create_shader_program(&self) -> programs::ShaderProgram {
        let mut shader_program = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateShaderProgram(self.internal.handle, &mut shader_program);
        }
        programs::ShaderProgram::new(shader_program, self.internal.clone())
    }

    pub fn set_render_passes(&self, child_nodes: &[&RenderPassNode], sort_order: &[i32]) {
        let mut handles: Vec<sys::Cobalt_RenderPassNode> =
            child_nodes.iter().map(|x| x.handle).collect();
        unsafe {
            sys::Cobalt_Renderer_SetRenderPasses(
                self.internal.handle,
                handles.as_mut_ptr(),
                handles.len(),
                sort_order.as_ptr(),
            );
        }
    }

    pub fn remove_all_render_passes(&self) {
        unsafe {
            sys::Cobalt_Renderer_RemoveAllRenderPasses(self.internal.handle);
        }
    }

    /// Obtain a copy of the associated graphics lock
    ///
    /// It is recommended not to use this function every time to access a lock.
    /// Instead, keep the lock for long-term use
    pub fn graphics_lock(&self) -> GraphicsLock {
        self.graphics_lock.clone()
    }

    /// Start rendering a new frame with the current render passes and their children.
    ///
    /// If a frame is currently rendering, this call will block until it's done.
    ///
    /// # Safety
    ///
    /// This library does not protect against all misuse of the renderer. Before
    /// starting a frame, YOU must ensure the following conditions are met.
    /// - All bound resources are alive. Any dropped objects that are still
    ///   bound to another object could cause a crash. For example, a dropped
    ///   ShaderProgram still in use by a ProgramNode.
    /// - No other renderer functions are running while this function is running.
    ///   A [`GraphicsLock`] obtained from [`Renderer::graphics_lock`] can be used to
    ///   handle this synchronization, or you can use your own mechanism.
    pub unsafe fn start_new_frame(&self) {
        // Acquiring the graphics lock exclusively to ensure no other graphics work is ongoing
        let _lock = self.graphics_lock.frame_lock();
        unsafe {
            sys::Cobalt_Renderer_StartNewFrame(self.internal.handle);
        }
    }

    /// Block until the current frame has finished drawing
    pub fn wait_for_draw_complete(&self) {
        unsafe {
            sys::Cobalt_Renderer_WaitForDrawComplete(self.internal.handle);
        }
    }

    /// Block until the current frame has finished drawing and then all
    /// output captures are done.
    ///
    /// When this function returns, all output captures can be read and should contain
    /// the content from the just completed frame
    pub fn wait_for_output_capture_complete(&self) {
        unsafe {
            sys::Cobalt_Renderer_WaitForOutputCaptureComplete(self.internal.handle);
        }
    }

    /// Block until all resources that were deferred for deletion are deleted
    pub fn wait_for_deferred_deletion_complete(&self) {
        unsafe {
            sys::Cobalt_Renderer_WaitForDeferredDeletionComplete(self.internal.handle);
        }
    }
}

impl Drop for RendererInternal {
    fn drop(&mut self) {
        unsafe {
            log::debug!("Deleting renderer");
            sys::Cobalt_Renderer_Delete(self.handle);
        }
    }
}

unsafe impl Send for RendererInternal {}
unsafe impl Sync for RendererInternal {}
