// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License

use cobalt_renderer::renderer::{
    DeviceEnumerationFlags, MemoryType, RendererInitializationFlags, WindowSystem,
};
use std::io::Write;
mod common;

#[test]
// Load all available plugins and check all device information
fn list_devices() {
    common::setup();

    let library = cobalt_renderer::init().unwrap();
    let mut enumerator = library.renderer_plugin_enumerator();

    let mut path = std::path::PathBuf::from(std::env::var("COBALT_SDK_PATH").unwrap());
    path = path.join("Bin/x64");
    enumerator.enumerate_plugins_in_directory(path).unwrap();

    for mut info in enumerator.all_plugins() {
        println!("Plugin: {}", info.display_name());
        println!("\tAPI     : {:?}", info.api_family());
        println!("\tVersion : {}", info.target_api_version());
        println!("\tName    : {}", info.name());
        std::io::stdout().flush().unwrap();

        let enumerator = info
            .create_device_enumerator(DeviceEnumerationFlags::None)
            .unwrap();

        let devices = enumerator.all_devices();
        println!("Device count: {}", devices.len());
        for d in devices.iter().enumerate() {
            println!("Device {}: {})", d.0, d.1.device_name());
            println!("\tDevice Vendor    : {}", d.1.vendor_name());
            println!("\tDevice Type      : {:?}", d.1.device_type());
            println!(
                "\tDedicated Memory : {} MB",
                d.1.memory_size_in_bytes(MemoryType::Dedicated) / (1024 * 1024)
            );
            println!(
                "\tShared Memory    : {} MB",
                d.1.memory_size_in_bytes(MemoryType::Shared) / (1024 * 1024)
            );
            print!("\tFeatures         : ");
            for f in d.1.all_supported_features() {
                print!("{f:?}\n\t                 : ");
            }
            println!();
        }

        let mut device = enumerator.preferred_device().expect("No preferred device");
        if let Err(e) = device.create_renderer(
            &[],
            &[],
            RendererInitializationFlags::None,
            WindowSystem::Headless,
        ) {
            log::error!("Failed to create renderer for device above");
        }
    }
}
