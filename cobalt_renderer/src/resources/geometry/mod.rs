// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

//! Vertex and index data for drawing
//!
//! Attributes represent the different fields for a vertex and sets of indices.
//! [`Renderables`][`crate::render_tree::RenderableNode`] bind to attributes to
//! know what to draw. Data for each attribute is stored in a backing buffer, either
//! a [`VertexBuffer`] or [`IndexBuffer`]
//! - Vertex buffers can hold one or more attributes
//! - Index buffers can only hold one attribute
//!
//! The process for setting up geometry data is
//! - Create the attributes
//! - Bind them to a buffer
//! - Set the initial data (optional)
//! - Allocate the buffer memory
//!
//! Data can be set through the attributes, in this case each attribute is separate in memory.
//! If data is set raw through the buffer, the attributes can be interleaved, depending on
//! how attributes are setup.

use bitflags::bitflags;

mod index_attribute;
mod index_buffer;
mod vertex_attribute;
mod vertex_buffer;

pub use index_attribute::*;
pub use index_buffer::*;
pub use vertex_attribute::*;
pub use vertex_buffer::*;

// NOTE: DTM Cobalt technically defines different performance hints
// and persistance flags for vertex and index attributes/buffers
// However, they are identical in practice and unlikely to change
// Therefor, we have a common set here for both, but this could
// change in the future if the two are not identical

bitflags! {
    /// Indicate how an attribute will be used on the GPU and the CPU
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
    /// Indicate how long a buffers data needs to persist
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
