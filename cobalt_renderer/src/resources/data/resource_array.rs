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
