// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

// TODO(DTM): Update

//! Cross-platform graphics rendering library
//!
//! The Cobalt Renderer is a generic 3D graphics library, intended for use
//! in any application requiring 3D visualization. It is a modern, cross-platform and light-weight,
//! library, featuring dynamically discoverable and loadable renderer plugins.
//!
//! A key design feature of this graphics library is that it abstracts away the details of
//! the underlying graphics API being used. Each renderer plugin drives a single underlying
//! graphics API, such as OpenGL, Direct3D, or Vulkan. This framework aims to unify concepts
//! and features across available graphics APIs, and present them in a consistent, easily
//! accessible form to the calling application. One important goal of this library is to
//! introduce minimal overhead, while allowing advanced optimizations that would be difficult
//! to achieve at the application level.
//!
//! This is a wrapper for the Cobalt Renderer written in C++. This Rust version
//! provides the complete Cobalt Renderer API with the safety of Rust. There are only a
//! few tweaks to the C++ API to make it safer and easier.
//!
//! For more information on how to use the Cobalt Renderer, please see the documentation
//! included in the SDK distribution. Included examples also show some basic usage.
//!
//! # Safety
//!
//! Cobalt will handle memory and appropriate deallocation of objects to ensure they
//! are done in the correct order. For example, even if the `Renderer` object is dropped
//! the actual C++ renderer will not be deleted until all graphics objects are dropped as well.
//!
//! However, there are no protections against improper API usage which will typically cause
//! Err results. If a C++ exception occurs there are no protections and the program will crash
//! without unwinding properly.
//!
//! Objects may also be bound to one another but this is not reflected in their lifetimes.
//! It is possible to drop an object still in use by another one.
//!  
//! # Threading
//!
//! Cobalt is designed with multi-threading in mind to maximize performance.
//! Modifying graphics content can be done across any number of threads.
//! Rendering is done on a separate thread and can occur while making changes.
//! Rust protects against all misuse of renderer objects with one clear exception.
//!
//! ### Graphics Lock
//!
//! All use of renderer objects MUST be while a graphics lock is held. No object methods,
//! or drops must occur while [`Renderer::start_new_frame`](crate::renderer::Renderer)
//! is running. See more under [`GraphicsLock`](crate::renderer::GraphicsLock)
//!
//! # Future work
//! - Holding a graphics lock automatically during all drops as they may be difficult to handle and lock against
//! - Make some objects more 'Rusty' with builder patterns and other type protections to prevent misuse

// Enum values in cobalt_renderer_sys are i32 on windows and u32 on linux
// We cast them to i32 regardless of platform when setting values in this crates enums
// This causes warnings on windows that this line disables
#![allow(clippy::unnecessary_cast)]

#[macro_use]
mod result;
pub use result::*;

mod library;
mod renderer_plugin;
mod renderer_plugin_enumerator;
pub use library::*;
pub use renderer_plugin::*;
pub use renderer_plugin_enumerator::*;

pub mod render_tree;
pub mod renderer;
pub mod resources;
