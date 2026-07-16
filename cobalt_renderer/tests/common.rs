// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

//! Utilities functions to reduce boilerplate in testing

#![allow(dead_code)]

use cobalt_renderer::renderer::*;
use cobalt_renderer::resources::textures::TextureBuffer;
use cobalt_renderer::resources::*;

pub struct CaptureContext {
    pub frame_buffer: frame_buffers::FrameBuffer,
    pub texture: textures::TextureBuffer2D,
    pub output: frame_buffers::FrameBufferOutput,
}

/// Initialize library and logger
pub fn setup_library() -> cobalt_renderer::Library {
    env_logger::init();
    cobalt_renderer::init().expect("Could not init cobalt_renderer library")
}

/// Initialize library and pick a preferred plugin
pub fn setup_plugin() -> cobalt_renderer::RendererPlugin {
    let library = setup_library();

    let mut enumerator = library.renderer_plugin_enumerator();
    let path = std::path::PathBuf::from(cobalt_renderer_sys::DEVELOPMENT_RUNTIME_BIN_DIR);
    enumerator
        .enumerate_plugins_in_directory(path)
        .expect("Could not enumerate plugins");
    enumerator
        .preferred_plugin()
        .expect("No preferred plugin found")
}

/// Initialize library, a plugin, pick a device with specified features and create a renderer
pub fn setup_renderer(
    required_features: &[cobalt_renderer::renderer::Feature],
) -> cobalt_renderer::renderer::Renderer {
    let mut plugin = setup_plugin();
    let mut enumerator = plugin
        .create_device_enumerator(DeviceEnumerationFlags::HeadlessRendering)
        .expect("Could not enumerate devices");
    enumerator.filter_devices_without_all_features(required_features);
    let mut device = enumerator.preferred_device().expect("No preferred device");
    device
        .create_renderer(
            &[],
            &[],
            RendererInitializationFlags::None,
            WindowSystem::Headless,
        )
        .expect("Couldn't create renderer")
}

/// Setup a framebuffer and render pass with output capture for testing
pub fn setup_frame_buffer_for_capture(renderer: &Renderer) -> CaptureContext {
    let mut texture = renderer.create_texture_buffer_2d();
    texture.set_usage_flags(textures::TextureUsageFlags::FrameBufferOutput);
    texture.set_texture_dimensions(&[1024, 1024], None);
    texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    texture
        .allocate_memory()
        .expect("Failed to allocate memory for texture buffer");

    // Create framebuffer
    let mut frame_buffer = renderer.create_frame_buffer();
    frame_buffer.define_viewport_region(&[0, 0], &[1024, 1024]);
    frame_buffer
        .bind_texture(&mut texture, frame_buffers::AttachmentType::Color, 0)
        .expect("Failed to bind texture buffer to frame buffer");

    let mut output = renderer.create_frame_buffer_output();
    frame_buffer.add_output_capture_target(&mut output, frame_buffers::AttachmentType::Color, 0);

    CaptureContext {
        frame_buffer,
        texture,
        output,
    }
}

/// Read framebuffer and save to file in $CWD/captures. File type is specified by file extension
pub fn save_capture(renderer: &Renderer, context: &mut CaptureContext, file_name: &str) {
    renderer.wait_for_output_capture_complete();

    let dimensions = context.output.image_dimensions();
    let mut data: Vec<u8> = vec![0; 4 * (dimensions[0] * dimensions[1]) as usize];
    context
        .output
        .read_buffer_data(
            data.as_mut_slice(),
            textures::SourceImageFormat::RGBA,
            textures::SourceDataFormat::UNorm8,
            &[0, 0],
            &dimensions,
        )
        .expect("Failed to read framebuffer");

    let mut path = std::env::current_dir().expect("Could not get current working directory");
    path = path.join("captures");
    std::fs::create_dir_all(&path)
        .expect("Could not create 'captures' directory in current working directory");

    path = path.join(file_name);
    image::save_buffer(
        path,
        data.as_slice(),
        dimensions[0],
        dimensions[1],
        image::ColorType::Rgba8,
    )
    .expect("Failed to save captured image");
}
