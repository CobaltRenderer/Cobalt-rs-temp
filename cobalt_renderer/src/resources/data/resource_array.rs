// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use bitflags::bitflags;

use cobalt_renderer_sys as sys;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PerformanceHint : u32 {
        const Default = sys::Cobalt_ResourceArrayPerformanceHint_Default as u32;
        const ReadNever = sys::Cobalt_ResourceArrayPerformanceHint_ReadNever as u32;
        const ReadRarely = sys::Cobalt_ResourceArrayPerformanceHint_ReadRarely as u32;
        const ReadOften = sys::Cobalt_ResourceArrayPerformanceHint_ReadOften as u32;
        const ReadFlagsMask = sys::Cobalt_ResourceArrayPerformanceHint_ReadFlagsMask as u32;
        const WriteNever = sys::Cobalt_ResourceArrayPerformanceHint_WriteNever as u32;
        const WriteRarely = sys::Cobalt_ResourceArrayPerformanceHint_WriteRarely as u32;
        const WriteOften = sys::Cobalt_ResourceArrayPerformanceHint_WriteOften as u32;
        const WriteFlagsMask = sys::Cobalt_ResourceArrayPerformanceHint_WriteFlagsMask as u32;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PersistenceFlags : u32 {
        const PersistAlways = sys::Cobalt_ResourceArrayDataPersistenceFlags_PersistAlways as u32;
        const InvalidateExistingDataOnWrite = sys::Cobalt_ResourceArrayDataPersistenceFlags_InvalidateExistingDataOnWrite as u32;
        const InvalidateExistingDataAfterDrawComplete = sys::Cobalt_ResourceArrayDataPersistenceFlags_InvalidateExistingDataAfterDrawComplete as u32;
    }
}

// This is a workaround for Rust not having generic specialization
// which would allow us to have different functions under the same name
// that would take different input types and have different implementations.
// Instead we have a trait which we only implement on some types
// which then have specializations. Then we can have one generic
// function which takes this trait and calls the specialized function
// on the type. Not ideal but functional

pub trait ResourceArray {
    #[doc(hidden)]
    fn array_handle(&mut self) -> sys::Cobalt_ResourceArray;

    fn set_performance_hints(
        &mut self,
        performance_hint_cpu: PerformanceHint,
        performance_hint_gpu: PerformanceHint,
    ) {
        unsafe {
            sys::Cobalt_ResourceArray_SetPerformanceHints(
                self.array_handle(),
                performance_hint_cpu.bits() as sys::Cobalt_ResourceArrayPerformanceHint,
                performance_hint_gpu.bits() as sys::Cobalt_ResourceArrayPerformanceHint,
            )
        }
    }

    fn set_data_persistence_flags(&mut self, data_persistence_flags: PersistenceFlags) {
        unsafe {
            sys::Cobalt_ResourceArray_SetDataPersistenceFlags(
                self.array_handle(),
                data_persistence_flags.bits() as sys::Cobalt_ResourceArrayDataPersistenceFlags,
            )
        }
    }
}
