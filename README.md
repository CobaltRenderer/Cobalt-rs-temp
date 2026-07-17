# Cobalt-rs

This is a Rust wrapper for the [Cobalt Renderer](https://github.com/CobaltRenderer/Cobalt). The Cobalt Renderer is a generic, cross platform graphics library. Cobalt aims to make using modern 3D graphics hardware simple. It does this by giving you a clean API to express what you want to draw (or compute), without getting bogged down in how to do that. Cobalt is designed to be an accessible, lightweight alternative to directly using low-level graphics APIs such as Vulkan, Direct3D and OpenGL.

This library provides the complete Cobalt Renderer API for Rust. There are only a few tweaks to the C++ API to make it safer and easier.

For more information on using this library, please see the documentation.

## Quick Start

To use this crate, the Cobalt Renderer SDK must be provided. In future, it will be automatically downloaded.
1. The SDK should be placed in the top level directory of the crate
2. The SDK directory should be named 'CobaltSDK'
3. The 'CobaltSDK' directory then contains directories 'Bin', 'Lib', etc

Once the SDK is provided, this crate may be used by any other project

## Example

To see general usage, please see `examples/hello_triangle.rs`. 
To run the example, use `cargo run --example hello_triangle`

## License

While originally developed in-house by Maptek, this work is now released as open source under the MIT license. Please see the [LICENSE.txt](LICENSE.txt)
file in this repository for full information.
