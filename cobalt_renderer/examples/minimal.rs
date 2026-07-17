// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

// This is a minimal example of rendering an RGB triangle to a texture
// The texture is then read back and saved to a PNG file
// For a complete example with a window, check out the `hello_triangle` example

// Cobalt Renderer
use cobalt_renderer::prelude::*;
use cobalt_renderer::renderer::{
    DeviceEnumerationFlags, RendererInitializationFlags, WindowSystem,
};
use cobalt_renderer::resources::*;

// HLSL shader programs for drawing our triangle, check out the file
const SHADER: &str = include_str!("minimal.hlsl");
// Size of texture
const TEXTURE_SIZE: [u32; 2] = [1024, 1024];
// Vertex data
const POSITIONS: [[f32; 2]; 3] = [[-0.649, -0.375], [0.649, -0.375], [0.0, 0.75]];
const COLORS: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

fn main() {
    // Cobalt renderer outputs lots of diagnostic information into logs
    // so we setup a log to listen for it (see 'log' crate)
    env_logger::init();

    // Initialize cobalt_renderer library
    let library = cobalt_renderer::init().unwrap();

    // First we need to load a renderer plugin
    // We will search through all the plugins in the SDK and
    // select the preferred one
    let path = std::path::PathBuf::from(cobalt_renderer::LOCAL_RUNTIME_BIN_DIR);
    let mut renderer_enumerator = library.renderer_plugin_enumerator();
    renderer_enumerator
        .enumerate_plugins_in_directory(path)
        .expect("Could not enumerate plugins");
    let mut plugin = renderer_enumerator
        .preferred_plugin()
        .expect("No plugin available");

    // With the plugin, we can enumerate the available devices
    let enumerator = plugin
        .create_device_enumerator(DeviceEnumerationFlags::None)
        .unwrap();

    // We could explore all available devices, inspect and filter them
    // but we'll just pick the preferred device (typically discrete and with most memory)
    let mut device = enumerator.preferred_device().expect("No preferred device");

    // We create the core renderer object!
    // We can enable features and select options here, and we need to tell
    // it what window system we are using
    let renderer = device
        .create_renderer(
            &[],
            &[],
            RendererInitializationFlags::None,
            WindowSystem::Headless,
        )
        .expect("Could not create renderer");

    // With the renderer, we can start building out what we want to draw

    // Create texture to back frame buffer
    let mut texture = renderer.create_texture_buffer_2d();
    texture.set_usage_flags(textures::TextureUsageFlags::FrameBufferOutput);
    texture.set_texture_dimensions(&TEXTURE_SIZE, None);
    texture.set_texture_format(textures::ImageFormat::RGBA, textures::DataFormat::UNorm8);
    texture
        .allocate_memory()
        .expect("Failed to allocate memory for texture buffer");

    // Create frame buffer output to capture texture contents after render
    let mut frame_buffer_output = renderer.create_frame_buffer_output();

    // Create frame buffer to render into
    let mut frame_buffer = renderer.create_frame_buffer();
    frame_buffer.define_viewport_region(&[0, 0], &TEXTURE_SIZE);
    frame_buffer
        .bind_texture(&mut texture, frame_buffers::AttachmentType::Color, 0)
        .expect("Could not bind texture to framebuffer");
    frame_buffer.add_output_capture_target(
        &mut frame_buffer_output,
        frame_buffers::AttachmentType::Color,
        0,
    );

    // Create render pass with dark grey clear color which renders
    // to the framebuffer
    let color: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
    let mut render_pass_node = renderer.create_render_pass_node();
    render_pass_node.bind_frame_buffer(&frame_buffer);
    render_pass_node.set_attachment_clear_data(frame_buffers::AttachmentType::Color, 0, &color);

    // Create shader program to render triangle
    // Shader is specified in `hello_triangle.hlsl` and has two
    // functions for the vertex and fragment stages
    let mut shader_program = renderer.create_shader_program();
    shader_program
        .load_shader_stage(
            programs::ShaderStage::Vertex,
            programs::ShaderSourceInfo::Hlsl {
                code: SHADER,
                entry_point_function_name: Some("vertex"),
            },
        )
        .expect("Failed to load vertex shader");
    shader_program
        .load_shader_stage(
            programs::ShaderStage::Fragment,
            programs::ShaderSourceInfo::Hlsl {
                code: SHADER,
                entry_point_function_name: Some("fragment"),
            },
        )
        .expect("Failed to load fragment shader");
    shader_program
        .compile_program()
        .expect("Failed to compile shader program");

    // Create program node that uses our shader program
    let mut program_node = renderer.create_program_node();
    program_node
        .bind_shader_program(&mut shader_program)
        .unwrap();

    // Create the state group node and any pipeline settings
    let mut state_group_node = renderer.create_state_group_node();
    state_group_node.set_polygon_fill_mode(cobalt_renderer::render_tree::PolygonFillMode::Solid);
    state_group_node.set_depth_test_enabled(false);

    // Attributes for positions and colors
    let mut position_attribute = renderer.create_vertex_attribute(
        geometry::VertexAttributeType::Float32,
        2,
        POSITIONS.len(),
        geometry::PerformanceHint::ReadNever | geometry::PerformanceHint::WriteNever,
        geometry::PerformanceHint::ReadOften | geometry::PerformanceHint::WriteNever,
        geometry::DataPersistenceFlags::PersistAlways,
    );
    let mut color_attribute = renderer.create_vertex_attribute(
        geometry::VertexAttributeType::Float32,
        3,
        COLORS.len(),
        geometry::PerformanceHint::ReadNever | geometry::PerformanceHint::WriteNever,
        geometry::PerformanceHint::ReadOften | geometry::PerformanceHint::WriteNever,
        geometry::DataPersistenceFlags::PersistAlways,
    );

    // Create vertex buffer to hold attributes
    let mut vertex_buffer = renderer.create_vertex_buffer();
    vertex_buffer
        .bind_vertex_attribute(&mut position_attribute)
        .unwrap();
    vertex_buffer
        .bind_vertex_attribute(&mut color_attribute)
        .unwrap();

    // Set the initial data and allocate the buffer memory
    position_attribute
        .set_initial_data(&POSITIONS, None)
        .unwrap();
    color_attribute.set_initial_data(&COLORS, None).unwrap();
    vertex_buffer.allocate_memory().unwrap();

    // Create a renderable to define what data to draw
    let mut renderable_node = renderer.create_renderable_node();
    renderable_node.set_vertex_count(3, 0, 0, 0).unwrap();
    renderable_node
        .set_primitive_mode(
            cobalt_renderer::render_tree::PrimitiveMode::Triangles,
            false,
            false,
        )
        .unwrap();

    // Bind the attributes to the renderables and the shader
    let position_id = shader_program.vertex_attribute_id("position").unwrap();
    let color_id = shader_program.vertex_attribute_id("color").unwrap();
    renderable_node
        .bind_vertex_attribute(&mut position_attribute, position_id)
        .unwrap();
    renderable_node
        .bind_vertex_attribute(&mut color_attribute, color_id)
        .unwrap();

    // Set up the render tree
    renderer.set_render_passes(&[&render_pass_node], &[1]);
    render_pass_node.add_child_node(&program_node, None);
    program_node.add_child_node(&state_group_node);
    state_group_node.add_child_node(&renderable_node);

    // Start a new frame
    unsafe {
        // Starting a frame is unsafe as we need to ensure
        // 2 conditions are met:
        // 1. No other renderer functions are called while this
        //    function is running. As we don't have other threads
        //    using the renderer, this is safe. There is also a
        //    lock to help prevent this in multithreaded applications
        // 2. All bound resources must be alive. Lifetimes are not
        //    tracked.
        renderer.start_new_frame();
    }

    // Wait until frame buffer output is ready
    renderer.wait_for_output_capture_complete();

    // Read frame buffer contents from output
    let mut data: Vec<u8> = vec![0; 4 * (TEXTURE_SIZE[0] * TEXTURE_SIZE[1]) as usize];
    frame_buffer_output
        .read_buffer_data(
            data.as_mut_slice(),
            textures::SourceImageFormat::RGBA,
            textures::SourceDataFormat::UNorm8,
            &[0, 0],
            &TEXTURE_SIZE,
        )
        .expect("Failed to read framebuffer");

    // Save texture to file
    let mut path = std::env::current_dir().expect("Could not get current working directory");
    path = path.join("output.png");
    image::save_buffer(
        &path,
        data.as_slice(),
        TEXTURE_SIZE[0],
        TEXTURE_SIZE[1],
        image::ColorType::Rgba8,
    )
    .expect("Failed to save captured image");

    println!("Rendered triangle saved to '{}'", path.display());

    // Everything cleans itself up at the end when dropped
}
