// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use bitflags::bitflags;
use std::sync::Arc;

use super::{DataArrayOutput, PerformanceHint, PersistenceFlags};
use crate::RendererResult;
use crate::renderer::RendererInternal;
use crate::resources::batching::TransferBatch;

use cobalt_renderer_sys as sys;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DataArrayUsageFlags : u32 {
        const Default = 0;
        const ShaderInput = sys::Cobalt_DataArrayUsageFlags_ShaderInput as u32;
        const ShaderOutput = sys::Cobalt_DataArrayUsageFlags_ShaderOutput as u32;
        const TransferSource = sys::Cobalt_DataArrayUsageFlags_TransferSource as u32;
        const TransferDestination = sys::Cobalt_DataArrayUsageFlags_TransferDestination as u32;
        const IndirectDrawSource = sys::Cobalt_DataArrayUsageFlags_IndirectDrawSource as u32;
        const IndirectDrawCountSource = sys::Cobalt_DataArrayUsageFlags_IndirectDrawCountSource as u32;
        const AtomicOperations = sys::Cobalt_DataArrayUsageFlags_AtomicOperations as u32;
    }
}

pub struct UnallocatedDataArray<'a> {
    data_array: DataArray,
    _initial_data: std::marker::PhantomData<&'a i32>,
}

impl<'a> UnallocatedDataArray<'a> {
    pub(crate) fn new(
        handle: sys::Cobalt_DataArray,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        UnallocatedDataArray {
            data_array: DataArray::new(handle, renderer_internal),
            _initial_data: std::marker::PhantomData,
        }
    }

    pub fn set_buffer_layout(
        &mut self,
        entry_stride_in_bytes: usize,
        entry_count: usize,
        has_counter: bool,
        counter_reset_value: u32,
    ) {
        unsafe {
            sys::Cobalt_DataArray_SetBufferLayout(
                self.data_array.handle,
                entry_stride_in_bytes,
                entry_count,
                if has_counter { 1 } else { 0 },
                counter_reset_value,
            )
        }
    }

    pub fn set_usage_flags(&mut self, usage_flags: DataArrayUsageFlags) {
        unsafe {
            sys::Cobalt_DataArray_SetUsageFlags(
                self.data_array.handle,
                usage_flags.bits() as sys::Cobalt_DataArrayUsageFlags,
            )
        }
    }

    pub fn set_initial_data<S: Sized>(&mut self, source_buffer: &'a [S]) -> RendererResult<()> {
        unsafe {
            return_on_failure!(sys::Cobalt_DataArray_SetInitialData(
                self.data_array.handle,
                source_buffer.as_ptr() as *const std::ffi::c_void,
                core::mem::size_of_val(source_buffer),
            ))
        }
        Ok(())
    }

    pub fn set_performance_hints(
        &mut self,
        performance_hint_cpu: PerformanceHint,
        performance_hint_gpu: PerformanceHint,
    ) {
        unsafe {
            sys::Cobalt_ResourceArray_SetPerformanceHints(
                self.data_array.handle as sys::Cobalt_ResourceArray,
                performance_hint_cpu.bits() as sys::Cobalt_ResourceArrayPerformanceHint,
                performance_hint_gpu.bits() as sys::Cobalt_ResourceArrayPerformanceHint,
            )
        }
    }

    pub fn set_data_persistence_flags(&mut self, data_persistence_flags: PersistenceFlags) {
        unsafe {
            sys::Cobalt_ResourceArray_SetDataPersistenceFlags(
                self.data_array.handle as sys::Cobalt_ResourceArray,
                data_persistence_flags.bits() as sys::Cobalt_ResourceArrayDataPersistenceFlags,
            )
        }
    }

    pub fn allocate_memory(self) -> RendererResult<DataArray> {
        unsafe {
            return_on_failure!(sys::Cobalt_DataArray_AllocateMemory(self.data_array.handle));
        }
        Ok(self.data_array)
    }
}

pub struct DataArray {
    pub(crate) handle: sys::Cobalt_DataArray,
    _renderer: Arc<RendererInternal>,
}

impl DataArray {
    pub(crate) fn new(
        handle: sys::Cobalt_DataArray,
        renderer_internal: Arc<RendererInternal>,
    ) -> Self {
        DataArray {
            handle,
            _renderer: renderer_internal,
        }
    }

    pub fn queue_data_update<S: Sized>(
        &mut self,
        source_buffer: &[S],
        target_buffer_offset: usize,
        transfer_batch: Option<&TransferBatch>,
    ) -> RendererResult<()> {
        let batch = match transfer_batch {
            None => std::ptr::null_mut(),
            Some(b) => b.handle,
        };
        unsafe {
            return_on_failure!(sys::Cobalt_DataArray_QueueDataUpdate(
                self.handle,
                source_buffer.as_ptr() as *const std::ffi::c_void,
                core::mem::size_of_val(source_buffer),
                target_buffer_offset,
                batch,
            ))
        }
        Ok(())
    }

    pub fn update_counter_reset_value(&mut self, counter_reset_value: u32) {
        unsafe { sys::Cobalt_DataArray_UpdateCounterResetValue(self.handle, counter_reset_value) }
    }

    pub fn queue_data_transfer(
        &mut self,
        target_buffer: &mut DataArray,
        transfer_count: usize,
        source_buffer_offset: usize,
        target_buffer_offset: usize,
        transfer_batch: Option<&TransferBatch>,
    ) -> RendererResult<()> {
        let batch = match transfer_batch {
            None => std::ptr::null_mut(),
            Some(b) => b.handle,
        };
        unsafe {
            return_on_failure!(sys::Cobalt_DataArray_QueueDataTransfer(
                self.handle,
                target_buffer.handle,
                transfer_count,
                source_buffer_offset,
                target_buffer_offset,
                batch,
            ))
        }
        Ok(())
    }

    pub fn add_output_capture_target(&mut self, output: &mut DataArrayOutput) {
        unsafe { sys::Cobalt_DataArray_AddOutputCaptureTarget(self.handle, output.handle) }
    }

    pub fn remove_output_capture_target(&mut self, output: &mut DataArrayOutput) {
        unsafe { sys::Cobalt_DataArray_RemoveOutputCaptureTarget(self.handle, output.handle) }
    }
}

impl Drop for DataArray {
    fn drop(&mut self) {
        unsafe {
            sys::Cobalt_DataArray_Delete(self.handle);
        }
    }
}

unsafe impl Send for DataArray {}
unsafe impl Sync for DataArray {}
