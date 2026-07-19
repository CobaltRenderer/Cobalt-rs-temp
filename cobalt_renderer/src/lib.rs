// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

//! The Cobalt Renderer is a generic, cross platform graphics library.
//! Cobalt aims to make using modern 3D graphics hardware simple.
//! It does this by giving you a clean API to express what you want to draw (or compute),
//! without getting bogged down in how to do that. Cobalt is designed to be an accessible,
//! lightweight alternative to directly using low-level graphics APIs such as Vulkan,
//! Direct3D and OpenGL.
//!
//! For more information on the Cobalt Renderer, check out the main
//! [Github](https://github.com/CobaltRenderer/Cobalt) page.
//!
//! Please see the C++ documentation included in the SDK distribution,
//! or the latest [online documentation](https://cobaltrenderer.github.io/Cobalt/?page=cobalt.graphics.Welcome)
//! for detailed information. Although for C++, types and methods map very closely to those in this crate.
//!
//! # Safety
//!
//! While Rust provides some safety guarantees, there are a few requirements that Rust will
//! not enforce that YOU must be aware of when using this library. `unsafe` is used to
//! indicate these areas.
//!
//! [`Renderer::start_new_frame`](crate::renderer::Renderer) kicks off the rendering process
//! on another thread. For the duration of the call, **no other library calls can be made**.
//! Your program must enforce this, either implicitly in it's threading structure, or explicitly
//! through locks. We provide a [`GraphicsLock`](crate::renderer::GraphicsLock) type which
//! can be obtained from a renderer at any point. `GraphicsLock`s all share the same lock
//! and can be used to protect against starting a frame. You are welcome to use your own
//! locking mechanism as well.
//!
//! We also don't protect against bound resources being dropped while in use by other
//! objects. For example, dropping the `ShaderProgram` bound to a `ProgramNode`.
//!
//! This crate does protect the core library objects (`Library`, `RendererPlugin` and `Renderer`)
//! staying alive for the lifetime of all other objects and being dropped in the appropriate order.
//!
//! The Cobalt Renderer does not raise C++ exceptions to report errors. An exception can be considered
//! equivalent to a Rust panic. Exceptions are not caught and may cause undefined behavior.
//!
//! # Building with the SDK
//!
//! `cobalt_renderer_sys` and subsequently `cobalt_renderer` relies on the Cobalt Renderer SDK
//! for builds and runtime dynamic libraries. There are a few options to use the SDK.
//! 1. (preferred) [Download a SDK](https://github.com/CobaltRenderer/Cobalt/releases) from GitHub
//!    or clone and build it yourself. Then set the environment variable `COBALT_SDK_DIR` to indicate
//!    where it is stored. For reliability and distribution, this is the best option. You can set the
//!    environment variable in your projects `.cargo/config.toml` file. For example, if the SDK was
//!    under the directory `CobaltSDK` at the root of the project directory.
//!    ```toml
//!    [env]
//!    COBALT_SDK_DIR = { value = "CobaltSDK", relative = true }
//!    ```
//!    You can also individually set `COBALT_INCLUDE_DIR`, `COBALT_LIB_DIR` and `COBALT_BIN_DIR` for
//!    the different subdirectories. All three must be set. `COBALT_SDK_DIR` will overwrite them.
//! 2. Use the `download_sdk` feature flag to have the `build.rs` script download a matching release
//!    for you. Downloads will by default be cached at `COBALT_SDK_CACHE_DIR`. If not set, the default
//!    location is at `$OUT_DIR/CobaltSDK`. It is HIGHLY recommended to cache the SDK outside your `target`
//!    directory as clean builds will remove the SDK and require them to be re-downloaded on the next build.
//!    Builds are not available for every platform and builds for Linux may have problems.
//! 3. Use the `build_sdk` feature flag to have the `build.rs` script clone and build the Cobalt
//!    Renderer project. Builds will be done in `$OUT_DIR/CobaltBuild` and the SDK stored at `COBALT_SDK_CACHE_DIR`.
//!    If not set, the default location is at `$OUT_DIR/CobaltSDK`. It is HIGHLY recommended to cache the SDK
//!    outside your `target` directory as clean builds will remove the SDK and require them the repo to be re-cloned
//!    and built again. Builds may require additional packages to be installed and on your `PATH`,
//!    particularly `cmake`. Please see the Cobalt Renderer
//!    [README](https://github.com/CobaltRenderer/Cobalt) for more information on building. Builds may take
//!    several minutes in which the `cobalt_renderer_sys(build)` target may appear to stall.
//!
//! ## Environment Variables
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
//! ## Versioning
//!
//! The `cobalt_renderer_sys` crate version corresponds with the versioning of SDK releases.
//! The major and minor versions must match and patch releases can differ. So for example,
//! version 2.0.0 of can use SDK version 2.0.0 or 2.0.1 (if it existed), but not version 2.1.0 or 3.0.0.
//!
//! This crate version does not correspond with the SDK version required. Please check `cobalt_renderer_sys`
//! dependency to know what SDK version is required.
//!
//! When using `build_sdk` or `download_sdk` features, the correct versions of the SDK will be downloaded/built.
//!
//! # Runtime Renderer Plugins
//!
//! During development, you can use [`LOCAL_RUNTIME_BIN_DIR`] to find renderer plugins. This constant
//! stores the Cobalt SDK binary directory which holds plugins.
//!
//! For distributions, you will need to distribute the renderer plugins and required shared libraries.
//! You cannot use [`LOCAL_RUNTIME_BIN_DIR`] for distributed builds and should use a different path.

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
