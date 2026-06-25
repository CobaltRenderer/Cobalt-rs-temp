// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

use cobalt_renderer::render_tree::*;
use cobalt_renderer::renderer::{
    DeviceEnumerationFlags, RendererInitializationFlags, WindowSystem,
};
use cobalt_renderer::resources::*;

use raw_window_handle::HasDisplayHandle;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopBuilder;
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;
#[cfg(target_os = "linux")]
use winit::platform::x11::EventLoopBuilderExtX11;
use winit::raw_window_handle::HasWindowHandle;
use winit::window::WindowBuilder;

// HLSL shader programs for drawing a triangle
// Vertex shader simply passes the position and color on as
// positions are already in correct screen coordinates
// Fragment shader just outputs the color
const SHADER: &str = "
struct VSInput {
    float3 position : position;
    float3 color : color;
};

struct VSOutput {
    float4 position : SV_POSITION;
    float3 color : COLOR;
};

float3 linearToSrgb(float3 lin) {
    float3 srgb;
    srgb.x = (lin.x <= 0.0031308) ? srgb.x = lin.x * 12.92 : 1.055 * pow(lin.x, 1.0 / 2.4) - 0.055;    
    srgb.y = (lin.y <= 0.0031308) ? srgb.y = lin.y * 12.92 : 1.055 * pow(lin.y, 1.0 / 2.4) - 0.055;    
    srgb.z = (lin.z <= 0.0031308) ? srgb.z = lin.z * 12.92 : 1.055 * pow(lin.z, 1.0 / 2.4) - 0.055;    
    return srgb;
}

VSOutput vertex(VSInput IN)
{
    VSOutput OUT;
    OUT.position = float4(IN.position, 1.0f);
    OUT.color = IN.color;
    return OUT;
}

float4 fragment(VSOutput IN) : SV_TARGET0
{
    return float4(linearToSrgb(IN.color), 1.0f);
}
";

const WINDOW_SIZE: [u32; 2] = [720, 480];

fn main() {
    // Cobalt renderer outputs lots of diagnostic information into logs
    // so we setup a log to listen for it (see 'log' crate)
    env_logger::init();

    // Use 'winit' crate to create a window to display stuff in
    let event_loop = EventLoopBuilder::new()
        .with_any_thread(true)
        .build()
        .unwrap();

    let window = WindowBuilder::new()
        .with_title("Cobalt Renderer Rust")
        .with_inner_size(PhysicalSize::new(WINDOW_SIZE[0], WINDOW_SIZE[1]))
        .build(&event_loop)
        .unwrap();

    // First we need to load the renderer plugin
    // We could dynamically search for a plugin or do anything here
    // But we know it should be adjacent to our program
    let mut path = std::path::PathBuf::from(std::env::var("COBALT_SDK_PATH").unwrap());
    path = path.join("Bin/x64");
    let library = cobalt_renderer::init().unwrap();
    let mut renderer_enumerator = library.renderer_plugin_enumerator();
    renderer_enumerator
        .enumerate_plugins_in_directory(path)
        .unwrap();
    let mut info = renderer_enumerator.preferred_plugin().unwrap();

    // We create a device enumerator to see what devices exist
    let enumerator = info
        .create_device_enumerator(DeviceEnumerationFlags::None)
        .unwrap();

    // We could explore all available devices, inspect and filter them
    // but we'll just pick the "best" device (typically discrete and with most memory)
    let mut device = enumerator.preferred_device().expect("No preferred device");

    // We create the core renderer object which comes with a graphics lock
    // to handle frame synchronizing across multiple threads
    let renderer = device
        .create_renderer(
            &[],
            &[],
            RendererInitializationFlags::None,
            WindowSystem::new_from_raw_display_handle(window.display_handle().unwrap().as_raw())
                .unwrap(),
        )
        .expect("Couldn't create renderer");

    let renderer_window = frame_buffers::Window::new_from_raw_handles(
        window.display_handle().unwrap().as_raw(),
        window.window_handle().unwrap().as_raw(),
    )
    .unwrap();

    // Create framebuffer which is bound to the window
    let mut frame_buffer = renderer.create_frame_buffer();
    frame_buffer.define_viewport_region(&[0, 0], &WINDOW_SIZE);
    frame_buffer
        .bind_window(
            renderer_window,
            &WINDOW_SIZE,
            frame_buffers::WindowDepthStencilMode::DepthFloat32,
            frame_buffers::WindowColorSpaceMode::Default,
            frame_buffers::WindowBindingFlags::None,
        )
        .unwrap();

    // Create render pass with grey clear color
    let color: cgmath::Vector4<f32> = cgmath::Vector4::new(0.2, 0.2, 0.2, 0.2);
    let mut render_pass_node = renderer.create_render_pass_node();
    render_pass_node.bind_frame_buffer(&frame_buffer);
    render_pass_node.set_attachment_clear_data(
        frame_buffers::AttachmentType::Color,
        0,
        color.as_ref(),
    );

    renderer.set_render_passes(&[&render_pass_node], &[1]);

    // Create shader program to render triangle
    let mut shader_program = renderer.create_shader_program();
    shader_program
        .load_shader_stage(
            programs::ShaderStage::Vertex,
            programs::ShaderSourceInfo::Hlsl {
                code: SHADER,
                entry_point_function_name: Some("vertex"),
            },
        )
        .unwrap();
    shader_program
        .load_shader_stage(
            programs::ShaderStage::Fragment,
            programs::ShaderSourceInfo::Hlsl {
                code: SHADER,
                entry_point_function_name: Some("fragment"),
            },
        )
        .unwrap();
    shader_program.compile_program().unwrap();

    // Create program node
    let mut program_node = renderer.create_program_node();
    program_node.bind_shader_program(&shader_program).unwrap();
    render_pass_node.add_child_node(&program_node, None);

    // Create the state group node and any pipeline settings
    let mut state_group_node = renderer.create_state_group_node();
    state_group_node.set_polygon_fill_mode(PolygonFillMode::Solid);
    state_group_node.set_depth_test_enabled(false);
    program_node.add_child_node(&state_group_node);

    // Create a renderable to define what data to draw
    let mut renderable = renderer.create_renderable_node();
    renderable.set_vertex_count(3, 0, 0, 0).unwrap();
    renderable
        .set_primitive_mode(PrimitiveMode::Triangles, false, false)
        .unwrap();

    // Vertex data for the triangle, positions and colors
    let positions = vec![
        cgmath::Vector3::new(0.0f32, 0.6, 0.5),
        cgmath::Vector3::new(0.6f32, -0.6, 0.5),
        cgmath::Vector3::new(-0.6f32, -0.6, 0.5),
    ];
    let colors = vec![
        cgmath::Vector3::new(1.0f32, 0.0, 0.0),
        cgmath::Vector3::new(0.0f32, 1.0, 0.0),
        cgmath::Vector3::new(0.0f32, 0.0, 1.0),
    ];

    // Attributes for positions and colors
    let mut position_attribute = renderer.create_vertex_attribute(
        geometry::VertexAttributeType::Float32,
        3,
        3,
        geometry::PerformanceHint::ReadNever | geometry::PerformanceHint::WriteRarely,
        geometry::PerformanceHint::ReadOften | geometry::PerformanceHint::WriteNever,
        geometry::DataPersistenceFlags::PersistAlways,
    );
    let mut color_attribute = renderer.create_vertex_attribute(
        geometry::VertexAttributeType::Float32,
        3,
        3,
        geometry::PerformanceHint::ReadNever | geometry::PerformanceHint::WriteRarely,
        geometry::PerformanceHint::ReadOften | geometry::PerformanceHint::WriteNever,
        geometry::DataPersistenceFlags::PersistAlways,
    );

    // Create vertex buffer to back attributes
    let mut vertex_buffer = renderer.create_vertex_buffer();
    vertex_buffer
        .bind_vertex_attribute(&mut position_attribute)
        .unwrap();
    vertex_buffer
        .bind_vertex_attribute(&mut color_attribute)
        .unwrap();

    // Set the initial data and allocate the buffer
    position_attribute
        .set_initial_data(&positions, None)
        .unwrap();
    color_attribute.set_initial_data(&colors, None).unwrap();

    vertex_buffer.allocate_memory().unwrap();

    // Bind the attributes to the renderables and the shader
    renderable
        .bind_vertex_attribute(
            &position_attribute,
            shader_program.vertex_attribute_id("position").unwrap(),
        )
        .unwrap();
    renderable
        .bind_vertex_attribute(
            &color_attribute,
            shader_program.vertex_attribute_id("color").unwrap(),
        )
        .unwrap();

    state_group_node.add_child_node(&renderable);

    // Main loop
    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, window_id } if window_id == window.id() => {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::RedrawRequested => {
                            // Start new frame
                            window.pre_present_notify();
                            unsafe {
                                renderer.start_new_frame();
                            }
                        }
                        WindowEvent::Resized(size) => {
                            frame_buffer
                                .notify_window_resized(&[size.width, size.height])
                                .unwrap();
                            frame_buffer
                                .define_viewport_region(&[0, 0], &[size.width, size.height]);
                        }
                        _ => (),
                    }
                }
                Event::AboutToWait => {
                    window.request_redraw();
                }

                _ => (),
            }
        })
        .unwrap();

    // Everything cleans itself up at the end when dropped
}
