use glium::Surface;
use glium::glutin;
use std::sync::{Arc, Mutex};

use super::super::pc9801vm::PC9801VM;
use super::crt::Crt;
use super::ram::GVRam;

#[derive(Copy, Clone)]
struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

glium::implement_vertex!(Vertex, position, color);

const VERTEX_SHADER_SRC: &str = r#"
        #version 140

        in vec2 position;
        in vec3 color;
        out vec3 vColor;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            vColor = color;
	}
    "#;

const FRAGMENT_SHADER_SRC: &str = r#"
        #version 140

        in vec3 vColor;
        out vec4 f_color;

        void main() {
            f_color = vec4(vColor, 1.0);
        }
    "#;

pub fn boot_display(machine: Arc<Mutex<PC9801VM>>) -> () {
    let mut crt = Crt::init();
    {
        let machine = machine.lock().unwrap();
        crt = machine.crt.cpy();
    }
    let dim = glium::glutin::dpi::LogicalSize::new(crt.scrx as f64, crt.scry as f64);
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(dim)
        .with_title(&*crt.title);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut events = Vec::new();
    let program = glium::Program::from_source(&display,
                                              VERTEX_SHADER_SRC,
                                              FRAGMENT_SHADER_SRC, None).unwrap();

    let refresh = || {
        let machine_ = machine.lock().unwrap();
        let mut vs = Vec::new();

        for y in 0..(crt.scry) {
            let fy = (y as f32)/(crt.scry as f32) * 2.0 - 1.0;
            for x in 0..(crt.scrx) {
                let fx = (x as f32)/(crt.scrx as f32) * 2.0 - 1.0;
                let GVRam(gvram) = machine_.graphics_ram;
                let (r, g, b) = crt.palette[gvram[x + (y * crt.scrx)] as usize];
                let (r, g, b) =
                    (((r as f32) / 255.0), ((g as f32)/ 255.0), ((b as f32)/255.0));
                vs.push(Vertex {position: [fx, fy], color: [r, g, b]});
            }
            //println!("fy:{}", fy);
        }

        let vertex_buffer = glium::VertexBuffer::new(&display, &vs).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();

        target.finish().unwrap();
    };

    'render: loop{
        events.clear();
        events_loop.poll_events(|event| { events.push(event); });
        if events.is_empty() {
            events_loop.run_forever(|event| {
                events.push(event);
                glutin::ControlFlow::Break
            });
        }
        for event in events.drain(..) {
            match event.clone() {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::CloseRequested => break 'render,
                        glutin::WindowEvent::Refresh => refresh(),
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    }
}
