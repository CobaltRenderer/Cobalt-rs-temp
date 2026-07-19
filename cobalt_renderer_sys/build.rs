// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

#![allow(unused)]

use std::env;
use std::path::{Path, PathBuf};

const SDK_PATH_VAR: &str = "COBALT_SDK_DIR";

const SDK_INCLUDE_VAR: &str = "COBALT_INCLUDE_DIR";
const SDK_LIB_VAR: &str = "COBALT_LIB_DIR";
const SDK_BIN_VAR: &str = "COBALT_BIN_DIR";

const SDK_BUILD_VAR: &str = "COBALT_SDK_BUILD_DIR";
const SDK_CACHE_VAR: &str = "COBALT_SDK_CACHE_DIR";

#[cfg(feature = "download_sdk")]
const SDK_DOWNLOADS: [(&str, SdkDownload); 4] = [
    (
        "macos-clang-arm64",
        SdkDownload {
            download_url: "https://github.com/CobaltRenderer/Cobalt/releases/download/v2.0.0/CobaltRenderer-SDK-macos-clang-arm64-v2.0.0.zip",
            hash: "f0db7acd3f7a1f27336861c4b223ad407a39b3040f75a87b14147274990f2b7f",
        },
    ),
    (
        "ubuntu-clang-arm64",
        SdkDownload {
            download_url: "https://github.com/CobaltRenderer/Cobalt/releases/download/v2.0.0/CobaltRenderer-SDK-ubuntu-clang-arm64-v2.0.0.zip",
            hash: "fe9283ab7f76252a66d9d39eabcb090d8861a4fbde62efc59bb4d09829172c64",
        },
    ),
    (
        "ubuntu-clang-x64",
        SdkDownload {
            download_url: "https://github.com/CobaltRenderer/Cobalt/releases/download/v2.0.0/CobaltRenderer-SDK-ubuntu-clang-x64-v2.0.0.zip",
            hash: "488dfc120f3ff28ac1ec3ffa46dd845e6c06c21a8a603dac70e7d43b3cc4c5c8",
        },
    ),
    (
        "windows-msvc-x64",
        SdkDownload {
            download_url: "https://github.com/CobaltRenderer/Cobalt/releases/download/v2.0.0/CobaltRenderer-SDK-windows-msvc-x64-v2.0.0.zip",
            hash: "2f9e0ce1bc8d52cfc2e3305024b701d78aba6fee39b37c90dc32851f7827041d",
        },
    ),
];

const SDK_GIT_REPO: &str = "https://github.com/CobaltRenderer/Cobalt.git";
const SDK_GIT_TAG: &str = "v2.0.0";

fn assert_sdk_version(sdk_path: impl AsRef<Path>) {
    let sdk_path = sdk_path.as_ref();

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
}

struct SdkPaths {
    lib: PathBuf,
    include: PathBuf,
    bin: PathBuf,
}

impl SdkPaths {
    fn from_sdk_dir(dir: impl AsRef<Path>) -> SdkPaths {
        assert_sdk_version(&dir);

        let dir = dir.as_ref();
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
            arch = Some("arm64");
        }
        let arch = arch.expect("Unsupported architecture. Must be x86, x86_64 or aarch64");
        SdkPaths {
            lib: dir.join("Lib").join(arch),
            include: dir.join("Include"),
            bin: dir.join("Bin").join(arch),
        }
    }
}

fn main() {
    let out_path: PathBuf = std::env::var("OUT_DIR").unwrap().into();

    // Determine SDK path
    let mut sdk_paths: Option<SdkPaths> = None;

    // SDK paths set by single SDK variable
    sdk_paths = sdk_paths.or_else(|| std::env::var_os(SDK_PATH_VAR).map(SdkPaths::from_sdk_dir));
    // SDK paths set by individual variables
    sdk_paths = sdk_paths.or_else(|| {
        let lib = std::env::var_os(SDK_LIB_VAR);
        let include = std::env::var_os(SDK_INCLUDE_VAR);
        let bin = std::env::var_os(SDK_BIN_VAR);
        if let Some(lib) = lib
            && let Some(include) = include
            && let Some(bin) = bin
        {
            Some(SdkPaths {
                lib: lib.into(),
                include: include.into(),
                bin: bin.into(),
            })
        } else {
            None
        }
    });
    // Using SDK build feature
    #[cfg(feature = "build_sdk")]
    {
        sdk_paths = sdk_paths.or_else(|| {
            // Check if SDK has already been downloaded

            let cache_path: PathBuf = match std::env::var_os(SDK_CACHE_VAR) {
                Some(p) => p.into(),
                None => out_path.join("CobaltSDK"),
            };
            if cache_path.exists() {
                return Some(SdkPaths::from_sdk_dir(cache_path));
            }

            let build_dir: PathBuf = match std::env::var_os(SDK_BUILD_VAR) {
                Some(p) => p.into(),
                None => out_path.join("CobaltBuild"),
            };
            if !build_dir.exists() {
                let repo = git2::Repository::clone(SDK_GIT_REPO, &build_dir)
                    .expect("Could not clone Git repo for build");
                let tag_ref = repo
                    .find_reference(&format!("refs/tags/{SDK_GIT_TAG}"))
                    .expect("Could not find expected version tag in Git repo");
                let obj = tag_ref
                    .peel(git2::ObjectType::Commit)
                    .expect("Could not peel Git reference");
                repo.checkout_tree(&obj, None)
                    .expect("Could not checkout Git repo");
            }

            let mut platform: Option<&str> = None;
            let mut arch: Option<&str> = None;
            #[cfg(target_os = "windows")]
            {
                platform = Some("windows-msvc");
            }
            #[cfg(target_os = "macos")]
            {
                platform = Some("macos-clang");
            }
            #[cfg(target_os = "linux")]
            {
                platform = Some("linux-clang");
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
            let platform = platform.expect("Unsupported platform for automatic SDK builds");
            let arch = arch.expect("Unsupported arch for automatic SDK builds");
            let preset = format!("{platform}-{arch}-release");

            let mut config_proc = std::process::Command::new("cmake")
                .current_dir(&build_dir)
                .arg("--preset")
                .arg(&preset)
                .arg("-DCOBALT_USE_SDL3=OFF")
                .arg("-DCOBALT_BUILD_TESTS=OFF")
                .arg("-DCOBALT_BUILD_EXAMPLES=OFF")
                .arg("-DCOBALT_CLANG_FORMAT_ON_ALL=OFF")
                .arg("-DCOBALT_CLANG_TIDY_ON_ALL=OFF")
                .arg("-DCOBALT_RUN_MSVC_STATIC_ANALYSIS=OFF")
                .spawn()
                .expect("Could not run cmake, please ensure cmake is available on your PATH");
            let config_result = config_proc.wait().expect("CMake didn't run");
            if !config_result.success() {
                panic!(
                    "Failed to configure Cobalt Renderer CMake project. Please see above for errors"
                );
            }

            let mut build_proc = std::process::Command::new("cmake")
                .current_dir(&build_dir)
                .arg("--build")
                .arg("--preset")
                .arg(&preset)
                .arg("--target")
                .arg("install")
                .spawn()
                .expect("Could not run cmake, please ensure cmake is available on your PATH");
            let build_result = build_proc.wait().expect("CMake didn't run");
            if !build_result.success() {
                panic!(
                    "Failed to build Cobalt Renderer CMake project. Please see above for errors"
                );
            }

            // Move SDK build to cache
            let build_out_path = build_dir.join("Output").join("SDK_RelWithDebInfo");
            assert_sdk_version(&build_out_path);
            if let Err(e) = std::fs::remove_dir_all(&cache_path)
                && e.kind() != std::io::ErrorKind::NotFound
            {
                panic!(
                    "Could not remove cache directory '{}', {}",
                    cache_path.display(),
                    e
                )
            }
            std::fs::rename(build_out_path, &cache_path)
                .expect("Could not move SDK build to cache location");

            Some(SdkPaths::from_sdk_dir(cache_path))
        });
    }
    // Using SDK download feature
    #[cfg(feature = "download_sdk")]
    {
        sdk_paths = sdk_paths.or_else(|| {
            // Check if SDK has already been downloaded
            let cache_path: PathBuf = match std::env::var_os(SDK_CACHE_VAR) {
                Some(p) => p.into(),
                None => out_path.join("CobaltSDK"),
            };
            if cache_path.exists() {
                return Some(SdkPaths::from_sdk_dir(cache_path));
            }

            // No SDK found, needs to be downloaded
            let sdk_download = find_sdk_download().expect("No SDK release could be found for the target platform. Consider using the `build_sdk` feature instead, or building and providing the SDK manually");
            download_verify_unzip_sdk(sdk_download, &cache_path);
            Some(SdkPaths::from_sdk_dir(cache_path))
        });
    }

    let sdk_paths = match sdk_paths {
        None => {
            panic!("
Cobalt Renderer SDK could not be found. Either
1. Download the Cobalt Renderer SDK and set environment variable '{}' to the SDK directory.
2. Enable feature 'download_sdk' on this crate. This will download the appropriate SDK and store it in the target directory.
3. Enable feature 'build_sdk' on this crate. This will clone and build the SDK from source and store it in the target directory.
See crate docs for more information.
   ", SDK_PATH_VAR);
        }
        Some(s) => s,
    };

    println!("SDK include path is '{}'", sdk_paths.include.display());
    println!("SDK lib path is '{}'", sdk_paths.lib.display());
    println!("SDK bin path is '{}'", sdk_paths.bin.display());

    // Verify paths exists
    if !sdk_paths.include.exists() {
        panic!(
            "Cobalt Renderer include path does not exist at expected path '{}'",
            sdk_paths.include.display(),
        )
    }
    if !sdk_paths.lib.exists() {
        panic!(
            "Cobalt Renderer lib path does not exist at expected path '{}'",
            sdk_paths.lib.display(),
        )
    }
    if !sdk_paths.bin.exists() {
        panic!(
            "Cobalt Renderer bin path does not exist at expected path '{}'",
            sdk_paths.bin.display(),
        )
    }

    // Tell cargo to look for static libraries in the specified directory
    println!(
        "cargo:rustc-link-search={}",
        sdk_paths.lib.to_str().expect("SDK path is not valid UTF-8")
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
    #[cfg(target_os = "macos")]
    {
        // Additional macOS links required
        println!("cargo:rustc-link-lib=dylib=c++");
    }

    // Import the following headers for the bindings
    let wrapper_contents = r#"
        #include <Cobalt/CBindings/CBindings.pkg>
        #include <Cobalt/CBindings/PlatformBindings.pkg>
    "#;

    // Generate bindings for the C bindings header
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", sdk_paths.include.display()))
        .header_contents("wrapper.h", wrapper_contents)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .array_pointers_in_arguments(true)
        .prepend_enum_name(false)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // We will then include this output in the crate
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Provide SDK binary directory for dependencies
    println!(
        "cargo::rustc-env=COBALT_RUNTIME_BIN_DIR={}",
        sdk_paths.bin.display()
    );

    // Rerun conditions
    println!("cargo:rerun-if-env-changed={SDK_PATH_VAR}");
    println!("cargo:rerun-if-env-changed={SDK_CACHE_VAR}");
    println!("cargo:rerun-if-env-changed={SDK_INCLUDE_VAR}");
    println!("cargo:rerun-if-env-changed={SDK_BIN_VAR}");
    println!("cargo:rerun-if-env-changed={SDK_LIB_VAR}");
    println!("cargo:rerun-if-changed={}", sdk_paths.include.display());
    println!("cargo:rerun-if-changed={}", sdk_paths.bin.display());
    println!("cargo:rerun-if-changed={}", sdk_paths.lib.display());
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
    let mut platform: Option<&str> = None;
    let mut toolchain: Option<&str> = None;
    let mut arch: Option<&str> = None;

    #[cfg(target_os = "windows")]
    {
        platform = Some("windows");
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

    let platform = platform?;
    let toolchain = toolchain?;
    let arch = arch?;
    let download_key = format!("{}-{}-{}", platform, toolchain, arch);

    SDK_DOWNLOADS
        .iter()
        .find(|d| d.0 == download_key)
        .map(|d| d.1.clone())
}

#[cfg(feature = "download_sdk")]
fn download_verify_unzip_sdk(sdk: SdkDownload, out_dir: &std::path::Path) {
    use sha2::Digest;
    use std::io::{Cursor, Read, Seek, Write};

    let response = ureq::get(sdk.download_url)
        .call()
        .expect("Failed to make network request for SDK");
    let status = response.status();
    if !status.is_success() {
        panic!(
            "Failed to download SDK from {}, status is {}",
            sdk.download_url, status
        );
    }
    let mut reader = response.into_body().into_reader();
    let mut sdk_file: Vec<u8> = vec![];
    reader
        .read_to_end(&mut sdk_file)
        .expect("Could not read full response body");

    // Verify download is correct
    let mut hasher = sha2::Sha256::new();
    hasher.update(&sdk_file);
    let hash = hasher.finalize();
    let actual_hash: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
    if actual_hash != sdk.hash {
        panic!(
            "Downloaded SDK has SHA256 hash of '{}', which does not match expected hash of '{}'",
            &actual_hash, sdk.hash
        );
    }

    // Unzip SDK
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(sdk_file))
        .expect("SDK download was not a valid ZIP archive");
    if let Err(e) = zip.extract(out_dir) {
        // Attempt to
        if let Err(e) = std::fs::remove_dir_all(out_dir) {
            println!(
                "Could not remove SDK directory '{}'. This may cause future build failures as path exists but contains invalid contents, {e}",
                out_dir.display()
            );
        }
        panic!(
            "SDK could not be extracted to '{}', {e} ",
            out_dir.display()
        );
    }
}
