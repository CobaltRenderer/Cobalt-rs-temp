// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

use std::f32::consts::PI;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopBuilder;
#[cfg(target_os = "linux")]
use winit::platform::wayland::EventLoopBuilderExtWayland;
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::WindowBuilder;

use cobalt_renderer::render_tree::*;
use cobalt_renderer::renderer::*;
use cobalt_renderer::resources::*;

const WINDOW_SIZE: [u32; 2] = [720, 480];

struct Context {
    renderer: Mutex<Renderer>,
    graphics_lock: GraphicsLock,
    render_pass: Mutex<RenderPassNode>,
    shutdown_signal: AtomicBool,
}

fn main() {
    // Cobalt renderer outputs lots of diagnostic information into logs
    // so we setup a log to listen for it (see 'log' crate)
    let logger = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .build();
    log::set_logger(Box::leak(Box::new(logger))).unwrap();
    log::set_max_level(log::LevelFilter::Info);

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

    let mut path = std::path::PathBuf::from(cobalt_renderer::DEVELOPMENT_RUNTIME_BIN_DIR);
    let library = cobalt_renderer::init().unwrap();
    let mut renderer_enumerator = library.renderer_plugin_enumerator();
    renderer_enumerator
        .enumerate_plugins_in_directory(path)
        .unwrap();
    let mut info = renderer_enumerator.preferred_plugin().unwrap();
    log::info!("Picked {:?}", info.api_family());

    let enumerator = info
        .create_device_enumerator(DeviceEnumerationFlags::None)
        .unwrap();
    let mut device = enumerator.preferred_device().expect("No preferred device");
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

    // Create framebuffer and render pass
    let mut frame_buffer = renderer.create_frame_buffer();
    frame_buffer.define_viewport_region(&[0, 0], &WINDOW_SIZE);
    unsafe {
        frame_buffer
            .bind_window(
                renderer_window,
                &WINDOW_SIZE,
                frame_buffers::WindowDepthStencilMode::None,
                frame_buffers::WindowColorSpaceMode::Default,
                frame_buffers::WindowBindingFlags::None,
            )
            .unwrap();
    }

    let mut render_pass_node = renderer.create_render_pass_node();
    render_pass_node.bind_frame_buffer(&frame_buffer);
    render_pass_node.set_attachment_clear_data(
        frame_buffers::AttachmentType::Color,
        0,
        &[0.0, 0.0, 0.0, 1.0],
    );

    renderer.set_render_passes(&[&render_pass_node], &[1]);

    let lock = renderer.graphics_lock();
    let context = Arc::new(Context {
        renderer: Mutex::new(renderer),
        graphics_lock: lock,
        render_pass: Mutex::new(render_pass_node),
        shutdown_signal: AtomicBool::new(false),
    });

    // Create other thread to do updates on
    let thread_context = context.clone();
    let thread = thread::spawn(move || {
        update_thread(thread_context);
    });

    // Main loop
    let main_context = context.clone();
    let start_time = Instant::now();
    let mut next_update = start_time + Duration::from_millis(16);
    event_loop
        .run(|event, elwt| {
            match event {
                Event::WindowEvent { event, window_id } if window_id == window.id() => {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::RedrawRequested => {
                            // Update throttling
                            if Instant::now() > next_update {
                                next_update = Instant::now() + Duration::from_millis(16);

                                window.pre_present_notify();
                                unsafe {
                                    main_context.renderer.lock().unwrap().start_new_frame();
                                }
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

    context.shutdown_signal.store(true, Ordering::SeqCst);
    thread.join().unwrap();

    drop(frame_buffer);
    context
        .renderer
        .lock()
        .unwrap()
        .wait_for_deferred_deletion_complete();
}

fn update_thread(context: Arc<Context>) {
    let start_time = Instant::now();
    while !context.shutdown_signal.load(Ordering::SeqCst) {
        // Update
        let t = start_time.elapsed().as_millis() as f32 / 1000.0;

        let r = (t.sin() + 1.0) / 2.0;
        let g = ((t + (0.666 * PI)).sin() + 1.0) / 2.0;
        let b = ((t + (1.333 * PI)).sin() + 1.0) / 2.0;

        let _lock = context.graphics_lock.lock();
        context
            .render_pass
            .lock()
            .unwrap()
            .set_attachment_clear_data(frame_buffers::AttachmentType::Color, 0, &[r, g, b, 1.0]);

        thread::sleep(Duration::from_millis(16));
    }
}
