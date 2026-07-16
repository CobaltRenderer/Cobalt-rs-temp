// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

// This is a complete example for a rotating RGB triangle in a window.
// - Uses winit to create a window and handle the event loop
// - Creates all resources and render tree nodes to draw an RGB triangle
// - Updates a state value to rotate the triangle each frame
// The interesting renderer stuff is in App::resumed. Much of the contents
// of this file is using winit to have a window and respond to events.
// For a simplified example, check out the `minimal` example

// Cobalt Renderer
use cobalt_renderer::prelude::*;
use cobalt_renderer::render_tree::*;
use cobalt_renderer::renderer::{
    DeviceEnumerationFlags, RendererInitializationFlags, WindowSystem,
};
use cobalt_renderer::resources::*;

// Winit
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes};

// Required traits to extract raw window and display handles from winit Window
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

// HLSL shader programs for drawing our triangle, check out the file
const SHADER: &str = include_str!("hello_triangle.hlsl");
// Default window size at startup
const WINDOW_SIZE: [u32; 2] = [720, 720];
// Vertex data
const POSITIONS: [[f32; 2]; 3] = [[-0.649, -0.375], [0.649, -0.375], [0.0, 0.75]];
const COLORS: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

fn main() {
    // Cobalt renderer outputs lots of diagnostic information into logs
    // so we setup a log to listen for it (see 'log' crate)
    env_logger::init();

    // Winit requires creating an App object which receives callbacks on startup,
    // on events and on exit. We'll create the event loop and let the App
    // handle the events.
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

// Winit app. Until we startup, `context` is None. On startup
// we will create a window and all the renderer objects needed
// and store them in `context`
#[derive(Default)]
struct App {
    context: Option<Context>,
}

// Context for the app, contains the window and all renderer objects we
// create and need to persist for the lifetime of the program
#[allow(dead_code)]
struct Context {
    window: Window,
    start_time: std::time::Instant,
    renderer: cobalt_renderer::renderer::Renderer,
    frame_buffer: Option<cobalt_renderer::resources::frame_buffers::FrameBuffer>,
    render_pass_node: cobalt_renderer::render_tree::RenderPassNode,
    shader_program: cobalt_renderer::resources::programs::ShaderProgram,
    program_node: cobalt_renderer::render_tree::ProgramNode,
    state_group_node: cobalt_renderer::render_tree::StateGroupNode,
    renderable_node: cobalt_renderer::render_tree::RenderableNode,
    vertex_buffer: cobalt_renderer::resources::geometry::VertexBuffer,
    position_attribute: cobalt_renderer::resources::geometry::VertexAttribute,
    color_attribute: cobalt_renderer::resources::geometry::VertexAttribute,
}

impl winit::application::ApplicationHandler for App {
    // This is effectively the startup of the application.
    // We will create the window and everything renderer related here!
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Only run if we haven't created our context yet
        if self.context.is_some() {
            return;
        }

        // Create a window
        let window_attributes = WindowAttributes::default()
            .with_title("Cobalt Renderer")
            .with_inner_size(winit::dpi::PhysicalSize::new(
                WINDOW_SIZE[0],
                WINDOW_SIZE[1],
            ));
        let window = event_loop
            .create_window(window_attributes)
            .expect("Could not create a window");

        // Initialize cobalt_renderer library
        let library = cobalt_renderer::init().unwrap();

        // First we need to load a renderer plugin
        // We will search through all the plugins in the SDK and
        // select the preferred one
        let path = std::path::PathBuf::from(cobalt_renderer::DEVELOPMENT_RUNTIME_BIN_DIR);
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
                WindowSystem::new_from_raw_display_handle(
                    window.display_handle().unwrap().as_raw(),
                )
                .unwrap(),
            )
            .expect("Could not create renderer");

        // With the renderer, we can start building out what we want to draw

        // Create framebuffer which is bound to the window
        let renderer_window = frame_buffers::Window::new_from_raw_handles(
            window.display_handle().unwrap().as_raw(),
            window.window_handle().unwrap().as_raw(),
        )
        .unwrap();

        let mut frame_buffer = renderer.create_frame_buffer();
        frame_buffer.define_viewport_region(&[0, 0], &WINDOW_SIZE);
        unsafe {
            // Binding a window is marked unsafe as we require the window
            // to be alive for the lifetime of the framebuffer (sse exiting method
            // for more), and we are passing in raw pointers
            frame_buffer
                .bind_window(
                    renderer_window,
                    &WINDOW_SIZE,
                    frame_buffers::WindowDepthStencilMode::DepthFloat32,
                    frame_buffers::WindowColorSpaceMode::Default,
                    frame_buffers::WindowBindingFlags::None,
                )
                .expect("Failed to bind window");
        }

        // Create render pass with dark grey clear color which renders
        // to the framebuffer
        let color:[f32;4] = [0.2, 0.2, 0.2, 1.0];
        let mut render_pass_node = renderer.create_render_pass_node();
        render_pass_node.bind_frame_buffer(&frame_buffer);
        render_pass_node.set_attachment_clear_data(
            frame_buffers::AttachmentType::Color,
            0,
            &color,
        );

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
        state_group_node.set_polygon_fill_mode(PolygonFillMode::Solid);
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
            .set_primitive_mode(PrimitiveMode::Triangles, false, false)
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

        // Store everything in the context
        self.context = Some(Context {
            window,
            start_time: std::time::Instant::now(),
            renderer,
            render_pass_node,
            frame_buffer: Some(frame_buffer),
            shader_program,
            program_node,
            state_group_node,
            renderable_node,
            vertex_buffer,
            position_attribute,
            color_attribute,
        });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Some(context) = self.context.as_mut() {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(size) => {
                    let frame_buffer = context.frame_buffer.as_mut().unwrap();
                    frame_buffer
                        .notify_window_resized(&[size.width, size.height])
                        .unwrap();
                    frame_buffer.define_viewport_region(&[0, 0], &[size.width, size.height]);
                }
                WindowEvent::RedrawRequested => {
                    // Update rotation state value
                    let rotation_state_id =
                        context.shader_program.state_value_id("rotation").unwrap();
                    let rotation = context.start_time.elapsed().as_secs_f32();
                    context
                        .renderable_node
                        .set_state_value(rotation_state_id, rotation, None);

                    // Notify winit we are about to draw a frame
                    context.window.pre_present_notify();

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
                        context.renderer.start_new_frame();
                    }

                    // Request we redraw again
                    context.window.request_redraw();
                }
                _ => {}
            }
        }
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(context) = self.context.as_mut() {
            // Delete framebuffer before we delete the window, otherwise we
            // may use after free. We also need to allow the renderer to
            // delete it as all renderer object deletion is deferred
            context.frame_buffer = None;
            context.renderer.wait_for_deferred_deletion_complete();
            // Everything else cleans itself up at the end when dropped now
        }
    }
}
