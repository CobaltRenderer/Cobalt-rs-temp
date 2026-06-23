// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License

use cobalt_renderer::renderer::{
    DeviceEnumerationFlags, RendererInitializationFlags, WindowSystem,
};
use cobalt_renderer::resources::*;

use cobalt_renderer::resources::textures::{TextureBuffer, TextureUsageFlags};

mod common;

#[test]
fn texture_types() {
    let (_library, mut info) = common::setup_plugin();

    let enumerator = info
        .create_device_enumerator(DeviceEnumerationFlags::None)
        .unwrap();

    let mut device = enumerator.preferred_device().expect("No preferred device");
    let renderer = device
        .create_renderer(
            &[],
            &[],
            RendererInitializationFlags::None,
            WindowSystem::Headless,
        )
        .expect("Couldn't create renderer");

    let mut output_texture = renderer.create_texture_buffer_2d();
    output_texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    output_texture.set_texture_dimensions(&[1024, 1024], None);
    output_texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    output_texture.allocate_memory().unwrap();

    let mut output_texture = renderer.create_texture_buffer_1d();
    output_texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    output_texture.set_texture_dimensions(1024, None);
    output_texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    output_texture.allocate_memory().unwrap();

    let mut output_texture = renderer.create_texture_buffer_3d();
    output_texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    output_texture.set_texture_dimensions(&[64, 64, 64], None);
    output_texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    output_texture.allocate_memory().unwrap();

    let mut output_texture = renderer.create_texture_buffer_cube();
    output_texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    output_texture.set_texture_dimensions(1024, None);
    output_texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    output_texture.allocate_memory().unwrap();

    let mut output_texture = renderer.create_texture_buffer_1d_array();
    output_texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    output_texture.set_texture_dimensions(1024, 10, None);
    output_texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    output_texture.allocate_memory().unwrap();

    let mut output_texture = renderer.create_texture_buffer_2d_array();
    output_texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    output_texture.set_texture_dimensions(&[1024, 1024], 5, None);
    output_texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    output_texture.allocate_memory().unwrap();

    let mut output_texture = renderer.create_texture_buffer_cube_array();
    output_texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    output_texture.set_texture_dimensions(1024, 3, None);
    output_texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    output_texture.allocate_memory().unwrap();
}
