// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

// TODO(DTM): Update

//! `cobalt_renderer_sys` provides C API bindings for the Cobalt Renderer. Use the `cobalt_renderer`
//! crate for a safer Rust API.
//! 
//! ### Building with the SDK
//!
//! `cobalt_renderer_sys` relies on the Cobalt Renderer SDK for builds and runtime dynamic libraries.
//! There are a few options to use the SDK.
//! 1. (preferred) [Download a SDK](https://github.com/CobaltRenderer/Cobalt/releases) from GitHub
//!    or clone and build it yourself. Then set the environment variable `COBALT_SDK_DIR` to indicate
//!    where it is stored. For reliability and distribution, this is the best option. You can set the
//!    environment variable in your projects `.cargo/config.toml` file. For example, if the SDK was
//!    under the directory `CobaltSDK` at the root of the project directory.
//!    ```toml
//!    [env]
//!    COBALT_SDK_PATH = { value = "CobaltSDK", relative = true }
//!    ```
//!    You can also individually set `COBALT_INCLUDE_DIR`, `COBALT_LIB_DIR` and `COBALT_BIN_DIR` for
//!    the different subdirectories. All three must be set and `COBALT_SDK_DIR` will overwrite them.
//! 2. Use the `download_sdk` feature flag to have the `build.rs` script download a matching release
//!    for you. Downloads will by default be cached at `COBALT_SDK_CACHE_DIR`. If not set, the default
//!    location is at `$OUT_DIR/CobaltSDK`. It is HIGHLY recommended to cache the SDK outside your `target`
//!    directory as clean builds will remove the SDK and require them to be re-downloaded on the next build.
//!    Builds are not available for every platform and builds for Linux may have problems.
//!    it is cached outside the `target` directory.
//! 3. Use the `build_sdk` feature flag to have the `build.rs` script clone and build the Cobalt
//!    Renderer project. Builds will be done in `$OUT_DIR/CobaltBuild` and the SDK stored at `COBALT_SDK_CACHE_DIR`.
//!    If not set, the default location is at `$OUT_DIR/CobaltSDK`. It is HIGHLY recommended to cache the SDK
//!    outside your `target` directory as clean builds will remove the SDK and require them the repo to be re-cloned
//!    and built again. Builds may require additional packages to be installed and on your `PATH`,
//!    particularly `cmake`. Please see the Cobalt Renderer
//!    [README](https://github.com/CobaltRenderer/Cobalt) for more information on building. Builds may take
//!    several minutes in which the `cobalt_renderer_sys(build)` target may appear to stall, especially for release builds.
//!
//! ### Environment Variables
//! 
//! | Variable | Default | Purpose |
//! |-|-|-|
//! | `COBALT_SDK_DIR` | None | Root directory of Cobalt SDK for builds |
//! | `COBALT_INCLUDE_DIR` | None | Include directory of Cobalt SDK for builds |
//! | `COBALT_LIB_DIR` | None | Lib directory of Cobalt SDK for builds |
//! | `COBALT_BIN_DIR` | None | Bin directory of Cobalt SDK for runtime during development |
//! | `COBALT_SDK_BUILD_DIR` | `$OUT_DIR/CobaltBuild` | Directory to perform SDK builds in |
//! | `COBALT_SDK_CACHE_DIR` | `$OUT_DIR/CobaltSDK` | Directory to store SDK downloads or build output |
//! 
//! ### Loading Renderer Plugins
//!
//! During development, you can use [`LOCAL_RUNTIME_BIN_DIR`] to find renderer plugins. This constant
//! stores the Cobalt SDK binary directory which holds plugins. 
//! 
//! For distributions, you will need to distribute the renderer plugins and required shared libraries.
//! You cannot use [`LOCAL_RUNTIME_BIN_DIR`] for distributed builds and should use a different path.

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

pub mod prelude;
