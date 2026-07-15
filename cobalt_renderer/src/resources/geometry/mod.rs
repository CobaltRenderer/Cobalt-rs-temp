// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

use bitflags::bitflags;

mod index_attribute;
mod index_buffer;
mod vertex_attribute;
mod vertex_buffer;

pub use index_attribute::*;
pub use index_buffer::*;
pub use vertex_attribute::*;
pub use vertex_buffer::*;

// NOTE(DTM): Cobalt technically defines different performance hints
// and persistence flags for vertex and index attributes/buffers
// However, they are identical in practice and unlikely to change
// Therefore, we have a common set here for both, but this could
// change in the future if the two are not identical

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PerformanceHint : u32 {
        const Default = 0x00000000;
        const ReadNever = 0x00000001;
        const ReadRarely = 0x00000002;
        const ReadOften = 0x00000004;
        const ReadFlagsMask = 0x000000FF;
        const WriteNever = 0x00000100;
        const WriteRarely = 0x00000200;
        const WriteOften = 0x00000400;
        const WriteFlagsMask = 0x0000FF00;
    }
}

impl Default for PerformanceHint {
    fn default() -> Self {
        PerformanceHint::Default
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DataPersistenceFlags : u32 {
        const PersistAlways = 0x00000000;
        const InvalidateExistingDataOnWrite = 0x000000001;
        const InvalidateExistingDataAfterDrawComplete = 0x000000002;
    }
}

impl Default for DataPersistenceFlags {
    fn default() -> Self {
        Self::PersistAlways
    }
}
