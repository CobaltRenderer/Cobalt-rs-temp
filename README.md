# Cobalt Rust Bindings

This is a Rust wrapper for the Cobalt Renderer. This Rust version provides the complete Cobalt Renderer API with the safety of Rust. There are only a few tweaks to the C++ API to make it safer and easier.

The Cobalt Renderer is a generic 3D graphics library, intended for use in any application requiring 3D visualization. It is a modern, cross-platform and light-weight, library, featuring dynamically discoverable and loadable renderer plugins.

For more information, please see the documentation (`cargo doc`)

## SDK Install

To use this crate, the Cobalt Renderer SDK must be provided. In future, it will be automatically downloaded.
1. The SDK should be placed in the top level directory of the crate
2. The SDK directory should be named 'CobaltSDK'
3. The 'CobaltSDK' directory then contains directories 'Bin', 'Lib', etc

Once the SDK is provided, this crate may be used by any other project

## Example

To see general usage, please see `examples/rgb_triangle.rs`. 
To run the example, use `cargo run --example rgb_triangle`


# License
While originally developed in-house by Maptek, this work is now released as open source under the MIT license. Please see the [LICENSE.txt](LICENSE.txt)
file in this repository for full information.
