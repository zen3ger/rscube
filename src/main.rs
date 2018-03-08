extern crate cgmath;
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate lyon;
extern crate resvg;

extern crate rscube;

use gfx::traits::{Device, FactoryExt};
use glutin::ElementState::Pressed;
use glutin::{Event, GlContext, VirtualKeyCode};
use lyon::tessellation::geometry_builder::{BuffersBuilder, VertexBuffers};
use lyon::tessellation::{FillOptions, FillTessellator, StrokeTessellator};
use resvg::tree::TreeExt;

use rscube::alg::parse::Parser;
use rscube::cube::Cube;
use rscube::render;
use rscube::render::*;

const RUBIKS_CUBE: &str = "gfx/rubiks_cube.svg";
const WINDOW_SIZE: f32 = 800.0;

fn main() {
    let mut dom = Dom::load(RUBIKS_CUBE);
    let mut cube = Cube::new();
    let mut parser = Parser::new();

    let mut fill_tess = FillTessellator::new();
    let mut stroke_tess = StrokeTessellator::new();
    let mut mesh = VertexBuffers::new();
    let mut transform = None;

    svg::tessellate(
        &dom,
        &mut fill_tess,
        &mut stroke_tess,
        &mut mesh,
        &mut transform,
    );

    let view_box = dom.rtree.svg_node().view_box;

    // get svg view box parameters
    let vb_width = view_box.size.width as f32;
    let vb_height = view_box.size.height as f32;
    let scale = vb_width / vb_height;

    // get x and y translation
    let (x_trans, y_trans) = if let Some(transform) = transform {
        (transform.e as f32, transform.f as f32)
    } else {
        (0.0, 0.0)
    };

    // set window scale
    let (width, height) = if scale < 1.0 {
        (WINDOW_SIZE, WINDOW_SIZE * scale)
    } else {
        (WINDOW_SIZE, WINDOW_SIZE / scale)
    };

    // init the scene object
    // use the viewBox, if available, to set the initial zoom and pan
    let pan = [vb_width / -2.0 + x_trans, vb_height / -2.0 + y_trans];
    let zoom = 2.0 / f32::max(vb_width, vb_height);
    let proj = cgmath::ortho(-scale, scale, -1.0, 1.0, -1.0, 1.0);
    let mut scene = Scene::new(zoom, pan, proj);

    // set up event processing and rendering
    let mut event_loop = glutin::EventsLoop::new();
    let glutin_builder = glutin::WindowBuilder::new()
        .with_dimensions(width as u32, height as u32)
        .with_decorations(true)
        .with_title("RsCube v0.1");

    let context = glutin::ContextBuilder::new().with_vsync(true);

    let (window, mut device, mut factory, mut main_fbo, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(glutin_builder, context, &event_loop);

    let shader = factory
        .link_program(
            render::VERTEX_SHADER.as_bytes(),
            render::FRAGMENT_SHADER.as_bytes(),
        )
        .unwrap();

    let pso = factory
        .create_pipeline_from_program(
            &shader,
            gfx::Primitive::TriangleList,
            gfx::state::Rasterizer::new_fill(),
            fill_pipeline::new(),
        )
        .unwrap();

    //let (vbo, ibo) = factory.create_vertex_buffer_with_slice(&mesh.vertices[..], &mesh.indices[..]);

    let mut cmd_queue: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let constants = factory.create_constant_buffer(1);

    let mut buf = String::with_capacity(5);

    'update: loop {
        let mut quit = false;
        event_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: glutin::WindowEvent::Closed,
                ..
            } => {
                quit = true;
            }
            Event::WindowEvent {
                event: glutin::WindowEvent::Resized(w, h),
                ..
            } => {
                let scl = w as f32 / h as f32;
                scene.update_proj(cgmath::ortho(-scl, scl, -1.0, 1.0, -1.0, 1.0));
            }
            Event::WindowEvent {
                event:
                    glutin::WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                state: Pressed,
                                virtual_keycode: Some(key),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                match key {
                    VirtualKeyCode::Escape => {
                        quit = true;
                    }
                    VirtualKeyCode::LBracket => {
                        scene.zoom *= 0.8;
                    }
                    VirtualKeyCode::RBracket => {
                        scene.zoom *= 1.2;
                    }
                    VirtualKeyCode::Space => {
                        if let Some(turns) = parser.parse(&buf).report().generate() {
                            for &t in &turns {
                                cube.turn(t);
                            }
                            dom.update(&cube);
                        }
                        buf.clear();
                        mesh.vertices.clear();
                        mesh.indices.clear();
                        svg::tessellate(
                            &dom,
                            &mut fill_tess,
                            &mut stroke_tess,
                            &mut mesh,
                            &mut transform,
                        );
                    }
                    VirtualKeyCode::Back => {
                        buf.clear();
                        cube = Cube::new();
                        dom.update(&cube);
                        mesh.vertices.clear();
                        mesh.indices.clear();
                        svg::tessellate(
                            &dom,
                            &mut fill_tess,
                            &mut stroke_tess,
                            &mut mesh,
                            &mut transform,
                        );
                    }
                    VirtualKeyCode::U => buf.push('U'),
                    VirtualKeyCode::D => buf.push('D'),
                    VirtualKeyCode::R => buf.push('R'),
                    VirtualKeyCode::L => buf.push('L'),
                    VirtualKeyCode::F => buf.push('F'),
                    VirtualKeyCode::B => buf.push('B'),
                    VirtualKeyCode::M => buf.push('M'),
                    VirtualKeyCode::E => buf.push('E'),
                    VirtualKeyCode::S => buf.push('S'),
                    VirtualKeyCode::I => buf.push('\''),
                    VirtualKeyCode::Key2 => buf.push('2'),
                    _key => {}
                };
            }
            _ => {}
        });

        let (vbo, ibo) =
            factory.create_vertex_buffer_with_slice(&mesh.vertices[..], &mesh.indices[..]);
        if quit {
            break;
        }

        gfx_window_glutin::update_views(&window, &mut main_fbo, &mut main_depth);

        cmd_queue.clear(&main_fbo.clone(), [1.0, 1.0, 1.0, 1.0]);

        cmd_queue.update_constant_buffer(&constants, &scene.into());
        cmd_queue.draw(
            &ibo,
            &pso,
            &fill_pipeline::Data {
                vbo: vbo.clone(),
                out_color: main_fbo.clone(),
                constants: constants.clone(),
            },
        );
        cmd_queue.flush(&mut device);

        window.swap_buffers().unwrap();

        device.cleanup();
    }
}
