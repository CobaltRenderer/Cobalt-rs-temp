<img width="300" src="https://github.com/CobaltRenderer/Cobalt/blob/main/Assets/logos/ColorTransparentText.png?raw=trueg">

# cobalt_renderer_sys

Using `bindgen`, this crate generates FFI bindings for the Cobalt Renderer CBindings library. Use the `cobalt_renderer` crate for a safer Rust API.

## Cobalt SDK

`cobalt_renderer_sys` requires the Cobalt Renderer SDK as an external dependency. The SDK can be provided by the user, automatically downloaded, or cloned and built from source. See documentation for more information.

## Versioning

This crates version corresponds to the SDK version in use. Versions must match the major and minor version numbers. Patch numbers may be different. When using the `download_sdk` and `build_sdk` features, the corresponding version will be used.

## Contributing

This project welcomes contributions and suggestions. Feel free to join our public [Discord server](https://discord.gg/St5bkhrnu8) to get involved. If you're interested in contributing code changes, please see the Cobalt Renderer [README](https://github.com/CobaltRenderer/Cobalt/tree/main#contributing) for guidance on how to make the process smoother.

## License

While originally developed in-house by Maptek, this work is now released as open source under the MIT license. Please see the [LICENSE.txt](LICENSE.txt) file in this repository for full information.
