// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

//! `cobalt_renderer_sys` provides C API bindings for the Cobalt Renderer. Use the `cobalt_renderer`
//! crate for a safer Rust API.
//!
//! # Building with the SDK
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
//! This crate version corresponds with the versioning of SDK releases. The major and minor versions must match
//! and patch releases can differ. So for example, version 2.0.0 of this crate can use SDK version 2.0.0
//! or 2.0.1 (if it existed), but not version 2.1.0 or 3.0.0.
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

// Bindings will have these issues which we will allow
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Binary directory where plugins can be found during local development
///
/// # IMPORTANT
///
/// This path is the absolute path to the bin directory of the Cobalt SDK
/// on the current machine. You should only use this variable for local builds.
/// For distribution your program should bundle the required shared libraries and access
/// them at a different path.
///
/// This is provided as a convenience for development and for examples/tests
pub const LOCAL_RUNTIME_BIN_DIR: &str = env!("COBALT_RUNTIME_BIN_DIR");
