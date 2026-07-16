// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

use cobalt_renderer::renderer::Feature;
use cobalt_renderer::resources::*;

use cobalt_renderer::resources::textures::{TextureBuffer, TextureUsageFlags};

mod common;

#[test]
fn texture_types() {
    // Test to create and allocate memory for all texture types

    let renderer = common::setup_renderer(&[Feature::TextureCubeArray]);

    let mut texture = renderer.create_texture_buffer_2d();
    texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    texture.set_texture_dimensions(&[1024, 1024], None);
    texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    texture.allocate_memory().unwrap();

    let mut texture = renderer.create_texture_buffer_1d();
    texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    texture.set_texture_dimensions(1024, None);
    texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    texture.allocate_memory().unwrap();

    let mut texture = renderer.create_texture_buffer_3d();
    texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    texture.set_texture_dimensions(&[64, 64, 64], None);
    texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    texture.allocate_memory().unwrap();

    let mut texture = renderer.create_texture_buffer_cube();
    texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    texture.set_texture_dimensions(1024, None);
    texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    texture.allocate_memory().unwrap();

    let mut texture = renderer.create_texture_buffer_1d_array();
    texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    texture.set_texture_dimensions(1024, 10, None);
    texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    texture.allocate_memory().unwrap();

    let mut texture = renderer.create_texture_buffer_2d_array();
    texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    texture.set_texture_dimensions(&[1024, 1024], 5, None);
    texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    texture.allocate_memory().unwrap();

    let mut texture = renderer.create_texture_buffer_cube_array();
    texture.set_usage_flags(TextureUsageFlags::FrameBufferOutput);
    texture.set_texture_dimensions(1024, 3, None);
    texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    texture.allocate_memory().unwrap();
}
