// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

use std::env;
use std::path::PathBuf;

const SDK_PATH_VAR: &str = "COBALT_SDK_PATH";

fn main() {
    // Determine SDK path
    let sdk_path: PathBuf = if let Some(p) = std::env::var_os(SDK_PATH_VAR) {
        // Set by environment variable
        p.into()
    } else {
        #[cfg(feature = "download_sdk")]
        {
            // Using SDK download feature
            let sdk_download = find_sdk_download().unwrap();
            let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
            let sdk_dir = out_dir.join("sdk");
            download_verify_unzip_sdk(sdk_download, &sdk_dir);
            sdk_dir
        }
        #[cfg(not(feature = "download_sdk"))]
        panic!("
Cobalt Renderer SDK could not be found. Either
1. Download the Cobalt Renderer SDK and set environment variable '{}' to the SDK directory.
   This can be done by setting the environment variable in .cargo/Config.toml in your project directory.
   See https://doc.rust-lang.org/cargo/reference/config.html#configuration-format for more details
2. Enable feature 'download_sdk' on this crate. This will download the appropriate SDK and store it
   in the target directory.
            ", SDK_PATH_VAR);
    };
    println!("SDK path is '{}'", sdk_path.display());

    // Verify path exists
    if !sdk_path.exists() {
        panic!(
            "Cobalt Renderer SDK does not exist at expected path '{}'",
            sdk_path.display(),
        )
    }

    // Verify version is supported
    let crate_version = env!("CARGO_PKG_VERSION");
    let crate_semvar = parse_semvar(crate_version).expect("Could not parse crate semvar");

    let sdk_version_file_path = sdk_path.join("version.yml");
    // NOTE(DTM): This is extremely rudimentary version checking but avoids bringing
    // in a dependency to parse YML files and semvar strings
    let sdk_version_yml = std::fs::read_to_string(sdk_version_file_path)
        .expect("Could not read SDK version.yml file");
    let sdk_version_yml_parts: Vec<&str> = sdk_version_yml.split(':').collect();
    assert_eq!(sdk_version_yml_parts.len(), 2, "Invalid version.yml file");
    assert_eq!(
        sdk_version_yml_parts[0].trim(),
        "version",
        "version.yml must contain 'version' field"
    );
    let sdk_version = sdk_version_yml_parts[1];
    let sdk_semvar = parse_semvar(sdk_version).expect("SDK version is not valid semvar");

    assert!(
        crate_semvar.0 == sdk_semvar.0 && crate_semvar.1 == sdk_semvar.1,
        "Unsupported Cobalt SDK version. This crate requires version {}.{} while provided SDK is version {}. Major and minor version numbers must match. Please use an SDK with the correct version or use a different version of this crate",
        crate_semvar.0,
        crate_semvar.1,
        sdk_version
    );

    // Determine paths in SDK
    #[allow(unused_assignments)]
    let mut arch: Option<&str> = None;
    #[cfg(target_arch = "x86_64")]
    {
        arch = Some("x64");
    }
    #[cfg(target_arch = "x86")]
    {
        arch = Some("x86");
    }
    #[cfg(target_arch = "aarch64")]
    {
        arch = Some("arm");
    }
    let arch = arch.expect("Unsupported architecture. Must be x86, x86_64 or aarch64");
    let lib_path = sdk_path.join("Lib").join(arch);
    let include_path = sdk_path.join("Include").join("Cobalt").join("CBindings");

    // Tell cargo to look for shared libraries in the specified directory
    println!(
        "cargo:rustc-link-search={}",
        lib_path.to_str().expect("SDK path is not valid UTF-8")
    );

    // Tell rustc to link to the CBindings library
    println!("cargo:rustc-link-lib=CobaltCBindingsStatic");
    #[cfg(target_os = "linux")]
    {
        // Additional linux links required
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-lib=dylib=c++abi");
        println!("cargo:rustc-link-lib=dylib=unwind");
    }

    // Import the following headers for the bindings
    let wrapper_contents = r#"
        #include <CBindings.pkg>
        #include <PlatformBindings.pkg>
    "#;

    // Generate bindings for the C bindings header
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", include_path.display()))
        .header_contents("wrapper.h", wrapper_contents)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .array_pointers_in_arguments(true)
        .prepend_enum_name(false)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // We will then include this output in the crate
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Rerun conditions
    println!("cargo:rerun-if-env-changed={SDK_PATH_VAR}");
    println!("cargo:rerun-if-changed={}", sdk_path.display());
}

// Very rudimentary semvar parsing, always assumes just 3 numbers
// No additional semvar details are supported
fn parse_semvar(semvar: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = semvar.trim().split(".").collect();
    if parts.len() != 3 {
        return None;
    }
    let major: u32 = parts[0].parse().ok()?;
    let minor: u32 = parts[1].parse().ok()?;
    let patch: u32 = parts[2].parse().ok()?;
    Some((major, minor, patch))
}

#[cfg(feature = "download_sdk")]
#[derive(Debug, PartialEq, Eq, Clone)]
struct SdkDownload {
    download_url: &'static str,
    hash: &'static str,
}

#[cfg(feature = "download_sdk")]
fn find_sdk_download() -> Option<SdkDownload> {
    // TODO(DTM): Doesn't include all releases
    let mut platform: Option<&str> = None;
    let mut toolchain: Option<&str> = None;
    let mut arch: Option<&str> = None;

    #[cfg(target_os = "windows")]
    {
        platform = Some("win");
        toolchain = Some("msvc");
    }
    #[cfg(target_os = "macos")]
    {
        platform = Some("macos");
        toolchain = Some("clang");
    }
    #[cfg(target_os = "linux")]
    {
        platform = Some("ubuntu");
        toolchain = Some("clang");
    }

    #[cfg(target_arch = "x86")]
    {
        arch = Some("x86");
    }
    #[cfg(target_arch = "x86_64")]
    {
        arch = Some("x64");
    }
    #[cfg(target_arch = "aarch64")]
    {
        arch = Some("arm64");
    }

    let downloads: std::collections::HashMap<&str, SdkDownload> = [
        // TODO(DTM): Fill out with official release
    ]
    .into();

    let platform = platform?;
    let toolchain = toolchain?;
    let arch = arch?;

    downloads
        .get(format!("{}-{}-{}", platform, toolchain, arch).as_str())
        .cloned()
}

#[cfg(feature = "download_sdk")]
fn download_verify_unzip_sdk(sdk: SdkDownload, out_dir: &std::path::Path) {
    // TODO(DTM): Not unzipping yet
    use sha2::Digest;
    use std::io::{Read, Seek, Write};

    let response = ureq::get(sdk.download_url).call().unwrap();
    let status = response.status();
    if !status.is_success() {
        panic!(
            "Failed to download SDK from {}, status is {}",
            sdk.download_url, status
        );
    }
    let mut reader = response.into_body().into_reader();
    let mut sdk_file: Vec<u8> = vec![];
    reader.read_to_end(&mut sdk_file).unwrap();

    let mut hasher = sha2::Sha256::new();
    hasher.update(&sdk_file);
    let hash = hasher.finalize();
    let hash_bytes = hash.to_vec();

    println!("actual {:?} = expected {}", hash_bytes, sdk.hash);

    todo!("Waiting for Cobalt release to test")
}
