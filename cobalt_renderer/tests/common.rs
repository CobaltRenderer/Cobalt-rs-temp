// Copyright (c) 2026, Maptek Pty Ltd 
// Licensed under the MIT License

#![allow(dead_code)]

use std::sync::atomic::AtomicBool;

use cobalt_renderer::renderer::*;
use cobalt_renderer::resources::textures::TextureBuffer;
use cobalt_renderer::resources::*;

static LOGGER_SETUP: AtomicBool = AtomicBool::new(false);
//static TEST_MUTEX: Mutex<std::option::Option<maptek_logging::LoggerGuard>> = Mutex::new(None);

pub fn setup() -> cobalt_renderer::Library {
    // We only want one test running at a time to prevent shared library
    // issues. It's not expected for tests to be able to run all at once
    //let mut guard = TEST_MUTEX.lock().unwrap();

    // if !LOGGER_SETUP.fetch_or(true, std::sync::atomic::Ordering::SeqCst) {
    //     *guard = Some(
    //         maptek_logging::builder()
    //             .console(true)
    //             .init()
    //             .unwrap()
    //     );
    // }

    cobalt_renderer::init().unwrap()
}

pub fn setup_plugin() -> (cobalt_renderer::Library, cobalt_renderer::RendererInfo) {
    let mut library = setup();

    let mut enumerator = library.renderer_plugin_enumerator();
    let mut path = std::path::PathBuf::from(std::env::var("COBALT_SDK_PATH").unwrap());
    path = path.join("Bin/x64");
    enumerator.enumerate_plugins_in_directory(path).unwrap();
    let info = enumerator.preferred_plugin().unwrap();
    (library, info)
}

pub struct CaptureContext {
    pub enumerator: GraphicsDeviceEnumerator,
    pub renderer: Renderer,
    pub frame_buffer: frame_buffers::FrameBuffer,
    pub texture: textures::TextureBuffer2D,
    pub output_capture: frame_buffers::FrameBufferOutput,
    pub renderer_info: cobalt_renderer::RendererInfo,
    pub library: cobalt_renderer::Library,
}

pub fn setup_capture() -> CaptureContext {
    let mut info = setup_plugin();

    // setup device, renderer, frame_buffer, capture, etc
    // so all the boiler plate is out of the way
    let enumerator = info
        .1
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

    let mut texture = renderer.create_texture_buffer_2d();
    texture.set_usage_flags(textures::TextureUsageFlags::FrameBufferOutput);
    texture.set_texture_dimensions(&[1024, 1024], None);
    texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    texture.allocate_memory().unwrap();

    // Create framebuffer
    let mut frame_buffer = renderer.create_frame_buffer();
    frame_buffer.define_viewport_region(&[0, 0], &[1024, 1024]);
    frame_buffer
        .bind_texture(&texture, frame_buffers::AttachmentType::Color, 0)
        .unwrap();

    let output_capture = renderer.create_frame_buffer_output();
    frame_buffer.add_output_capture_target(
        &output_capture,
        frame_buffers::AttachmentType::Color,
        0,
    );

    CaptureContext {
        library: info.0,
        renderer_info: info.1,
        enumerator,
        renderer,
        frame_buffer,
        texture,
        output_capture,
    }
}

pub fn save_capture(context: &mut CaptureContext, file_name: &str) {
    context.renderer.wait_for_output_capture_complete();

    let dimensions = context.output_capture.image_dimensions();
    let mut data: Vec<u8> = vec![0; 4 * (dimensions[0] * dimensions[1]) as usize];
    context
        .output_capture
        .read_buffer_data(
            data.as_mut_slice(),
            textures::SourceImageFormat::RGBA,
            textures::SourceDataFormat::UNorm8,
            &[0, 0],
            &dimensions,
        )
        .unwrap();

    let mut path = std::env::current_dir().unwrap();
    path = path.join("captures");
    std::fs::create_dir_all(&path).unwrap();

    path = path.join(file_name);
    image::save_buffer(
        path,
        data.as_slice(),
        dimensions[0],
        dimensions[1],
        image::ColorType::Rgba8,
    )
    .unwrap();
}
