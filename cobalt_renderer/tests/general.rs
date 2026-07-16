// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

use cobalt_renderer::render_tree::*;
use cobalt_renderer::resources::*;
use common::save_capture;

mod common;

#[repr(C)]
struct Vertex {
    position: [f32;3],
    color: [f32;3],
}

impl Vertex {
    fn new(xyz: (f32, f32, f32), rgb: (f32, f32, f32)) -> Vertex {
        Vertex {
            position: [xyz.0, xyz.1, xyz.2],
            color: [rgb.0, rgb.1, rgb.2],
        }
    }
}

const SHADER: &str = "
struct VSInput {
    float3 position : position;
    float3 color : color;
};

struct VSOutput {
    float4 position : SV_POSITION;
    float3 color : COLOR;
};

VSOutput vertex(VSInput IN)
{
    VSOutput OUT;

    OUT.color = IN.color;
    OUT.position = float4(IN.position.xy, IN.position.z, 1.0f);

    return OUT;
}

float4 fragment(VSOutput IN) : SV_TARGET0
{
    return float4(IN.color, 1.0f);
}
";

#[test]
fn general() {
    // Basic test to render a hexagon and save it as an image

    let renderer = common::setup_renderer(&[]);
    let mut capture = common::setup_frame_buffer_for_capture(&renderer);

    let color = [0.0f32, 0.0, 0.0, 0.0];
    let mut render_pass_node = renderer.create_render_pass_node();
    render_pass_node.bind_frame_buffer(&capture.frame_buffer);
    render_pass_node.set_attachment_clear_data(
        frame_buffers::AttachmentType::Color,
        0,
        &color,
    );
    renderer.set_render_passes(&[&render_pass_node], &[1]);

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

    let mut program_node = renderer.create_program_node();
    program_node
        .bind_shader_program(&mut shader_program)
        .unwrap();
    render_pass_node.add_child_node(&program_node, None);

    let mut state_group_node = renderer.create_state_group_node();
    state_group_node.set_polygon_fill_mode(PolygonFillMode::Solid);
    state_group_node.set_depth_test_enabled(false);
    program_node.add_child_node(&state_group_node);

    let mut renderable = renderer.create_renderable_node();
    renderable.set_vertex_count(3, 0, 0, 0).unwrap();
    renderable
        .set_primitive_mode(PrimitiveMode::Triangles, false, false)
        .unwrap();

    let vertices = vec![
        Vertex::new((0.0, 0.0, 0.0), (1.0, 1.0, 1.0)),
        Vertex::new((1.0, 0.0, 0.0), (1.0, 0.0, 0.0)),
        Vertex::new((0.5, 0.86, 0.0), (1.0, 1.0, 0.0)),
        Vertex::new((-0.5, 0.86, 0.0), (0.0, 1.0, 0.0)),
        Vertex::new((-1.0, 0.0, 0.0), (0.0, 1.0, 1.0)),
        Vertex::new((-0.5, -0.86, 0.0), (0.0, 0.0, 1.0)),
        Vertex::new((0.5, -0.86, 0.0), (1.0, 0.0, 1.0)),
    ];

    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1];

    let mut position_attribute = renderer.create_vertex_attribute(
        geometry::VertexAttributeType::Float32,
        3,
        vertices.len(),
        geometry::PerformanceHint::ReadNever | geometry::PerformanceHint::WriteRarely,
        geometry::PerformanceHint::ReadOften | geometry::PerformanceHint::WriteNever,
        geometry::DataPersistenceFlags::PersistAlways,
    );
    let mut color_attribute = renderer.create_vertex_attribute(
        geometry::VertexAttributeType::Float32,
        3,
        vertices.len(),
        geometry::PerformanceHint::ReadNever | geometry::PerformanceHint::WriteRarely,
        geometry::PerformanceHint::ReadOften | geometry::PerformanceHint::WriteNever,
        geometry::DataPersistenceFlags::PersistAlways,
    );
    let mut index_attribute = renderer.create_index_attribute(
        geometry::IndexAttributeType::UInt32,
        indices.len(),
        geometry::PerformanceHint::ReadNever | geometry::PerformanceHint::WriteRarely,
        geometry::PerformanceHint::ReadOften | geometry::PerformanceHint::WriteNever,
        geometry::DataPersistenceFlags::PersistAlways,
    );

    let mut vertex_buffer = renderer.create_vertex_buffer();
    vertex_buffer
        .bind_vertex_attribute_manual_layout(
            &mut position_attribute,
            0,
            std::mem::size_of::<Vertex>(),
        )
        .unwrap();
    vertex_buffer
        .bind_vertex_attribute_manual_layout(
            &mut color_attribute,
            12,
            std::mem::size_of::<Vertex>(),
        )
        .unwrap();
    vertex_buffer.set_raw_initial_data(&vertices).unwrap();
    vertex_buffer.allocate_memory().unwrap();

    let mut index_buffer = renderer.create_index_buffer();
    index_buffer
        .bind_index_attribute(&mut index_attribute)
        .unwrap();
    index_attribute.set_initial_data(&indices, None).unwrap();
    index_buffer.allocate_memory().unwrap();

    renderable
        .bind_vertex_attribute(
            &mut position_attribute,
            shader_program.vertex_attribute_id("position").unwrap(),
        )
        .unwrap();
    renderable
        .bind_vertex_attribute(
            &mut color_attribute,
            shader_program.vertex_attribute_id("color").unwrap(),
        )
        .unwrap();
    renderable
        .bind_index_attribute(&mut index_attribute)
        .unwrap();
    renderable.set_vertex_count(indices.len(), 0, 0, 0).unwrap();

    state_group_node.add_child_node(&renderable);

    // Main loop

    unsafe {
        renderer.start_new_frame();
    }

    save_capture(&renderer, &mut capture, "general.png");
}
