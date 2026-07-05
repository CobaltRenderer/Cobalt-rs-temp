// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

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
//! ### Development
//!
//!
//!
//! ### Deployment

// Bindings will have these issues which we will allow
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const DEVELOPMENT_RUNTIME_BIN_DIR: &str = env!("RUNTIME_BIN_DIR");
