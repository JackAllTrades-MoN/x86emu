use glium::Surface;
use glium::glutin;
use std::sync::{Arc, Mutex};

const SCRX: usize = 640;
const SCRY: usize = 400;
const TITLE: &str = "x86 emulator";

pub type VRAM = [u8; SCRX * SCRY];

pub type RGB = (u8, u8, u8);

pub struct Display {
    palette: [RGB; 16], // 16 colors palette
}

pub fn gen_vram() -> VRAM {
    [0x02; SCRX * SCRY]
}

#[derive(Copy, Clone)]
struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

glium::implement_vertex!(Vertex, position, color);

impl Display {
    pub fn init() -> Display {
        Display { palette: [(0x00, 0x00, 0x00), // black
                            (0xff, 0xff, 0xff), // white
                            (0xc0, 0xc0, 0xc0), // silver
                            (0x80, 0x00, 0x00), // maroon
                            (0x80, 0x00, 0x80), // purple
                            (0x00, 0x80, 0x00), // green
                            (0x80, 0x80, 0x00), // olive
                            (0x00, 0x00, 0x80), // navy
                            (0x00, 0x80, 0x80), // teal
                            (0x80, 0x80, 0x80), // gray
                            (0xff, 0x00, 0x00), // red
                            (0xff, 0x00, 0xff), // fuchsia
                            (0x00, 0xff, 0x00), // lime
                            (0xff, 0xff, 0x00), // yellow
                            (0x00, 0x00, 0xff), // blue
                            (0x00, 0xff, 0xff), // aqua
        ], }
    }
    pub fn boot_display(self, vram: Arc<Mutex<VRAM>>) -> () {
        let dim = glium::glutin::dpi::LogicalSize::new(SCRX as f64, SCRY as f64);
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_dimensions(dim)
            .with_title(TITLE);
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        let mut events = Vec::new();

        let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec3 color;
        out vec3 vColor;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            vColor = color;
	}
    "#;

        let fragment_shader_src = r#"
        #version 140

        in vec3 vColor;
        out vec4 f_color;

        void main() {
            f_color = vec4(vColor, 1.0);
        }
    "#;

        let program = glium::Program::from_source(&display,
                                                  vertex_shader_src,
                                                  fragment_shader_src, None).unwrap();

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
                            _ => (),
                        }
                    },
                    _ => (),
                }
            }

            let vram_ = vram.lock().unwrap();
            let mut vs = Vec::new();

            let offset_x = 1.0;
            let offset_y = 1.0;

            for y in 0..SCRY {
                let fy = (y as f32)/(SCRY as f32) * 2.0 - offset_y;
                for x in 0..SCRX {
                    let fx = (x as f32)/(SCRX as f32) * 2.0 - offset_x;
                    let (r, g, b) = self.palette[vram_[x + (y * SCRX)] as usize];
                    let (r, g, b) =
                        (((r as f32) / 255.0), ((g as f32)/ 255.0), ((b as f32)/255.0));
                    vs.push(Vertex {position: [fx, fy], color: [r, g, b]});
                }
                println!("fy:{}", fy);
            }

            let vertex_buffer = glium::VertexBuffer::new(&display, &vs).unwrap();
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);
            target.draw(&vertex_buffer, &indices, &program,
                        &glium::uniforms::EmptyUniforms,
                        &Default::default()).unwrap();

            target.finish().unwrap();
        }
    }
}
