<img width="300" src="https://github.com/CobaltRenderer/Cobalt/blob/main/Assets/logos/ColorTransparentText.png?raw=trueg">

# Cobalt-rs

The Cobalt Renderer is a generic, cross platform graphics library. Cobalt aims to make using modern 3D graphics hardware simple. It does this by giving you a clean API to express what you want to draw (or compute), without getting bogged down in how to do that. Cobalt is designed to be an accessible, lightweight alternative to directly using low-level graphics APIs such as Vulkan, Direct3D and OpenGL.

This is a Rust wrapper for the [Cobalt Renderer](https://github.com/CobaltRenderer/Cobalt). The `cobalt_renderer` crate provides the complete Cobalt Renderer API for Rust. There are only a few tweaks to the C++ API to make it safer and easier.

## Cobalt SDK

`cobalt_renderer` requires the Cobalt Renderer SDK as an external dependency. The SDK can be provided by the user, automatically downloaded, or cloned and built from source. See documentation for more information.

## Getting Started 

The fastest way to start using the crate is to add it as a dependency with the `download_sdk` feature, which will attempt to download an official release for your target platform.

```toml
[dependencies]
cobalt_renderer = { version = "0.1.0", features = ["download_sdk"] }
```

You should also set the `COBALT_SDK_CACHE_DIR` environment variable in your projects `.cargo/config.toml` to save the SDK outside the `target` directory and preserve it when `cargo clean` is run.

```toml
[env]
COBALT_SDK_CACHE_DIR = { value = "CobaltSDK", relative = true }
```

IF an SDK is not available for your platform or causes issues, you can also use the `build_sdk` feature, which will clone and build the SDK from source.

## Example

To see general usage, please see `examples/hello_triangle.rs`. 
To run the example, use `cargo run --example hello_triangle`.

## Contributing

This project welcomes contributions and suggestions. Feel free to join our public [Discord server](https://discord.gg/St5bkhrnu8) to get involved. If you're interested in contributing code changes, please see the Cobalt Renderer [README](https://github.com/CobaltRenderer/Cobalt/tree/main#contributing) for guidance on how to make the process smoother.

## License

While originally developed in-house by Maptek, this work is now released as open source under the MIT license. Please see the [LICENSE.txt](https://github.com/CobaltRenderer/Cobalt-rs/blob/main/LICENSE.txt) file in this repository for full information.
