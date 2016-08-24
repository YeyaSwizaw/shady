#[macro_use] extern crate glium;
extern crate notify;
extern crate shady_script;

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::sync::mpsc::channel;

use glium::{Program, VertexBuffer, DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;

use notify::{RecommendedWatcher, Watcher};

#[derive(Copy, Clone)]
struct Vertex {
    v_xy: [f32; 2],
    v_uv: [f32; 2]
}

implement_vertex!(Vertex, v_xy, v_uv);

#[derive(Debug, Clone)]
pub struct ImageSource(String);

static vertex_shader_source: &'static str = r#"
    #version 330 core

    in vec2 v_xy;
    in vec2 v_uv;

    out vec2 uv;

    void main() {
        gl_Position = vec4(v_xy, 0, 1);
        uv = v_uv;
    }
"#;

static shape: [Vertex; 4] = [
    Vertex { v_xy: [-1.0, -1.0], v_uv: [0.0, 0.0] },
    Vertex { v_xy: [ 1.0, -1.0], v_uv: [1.0, 0.0] },
    Vertex { v_xy: [ 1.0,  1.0], v_uv: [1.0, 1.0] },
    Vertex { v_xy: [-1.0,  1.0], v_uv: [0.0, 1.0] },
];

struct ImageDisplay {
    display: GlutinFacade,
    buffer: VertexBuffer<Vertex>,
    program: Program,
    done: bool
}

#[derive(Debug)]
enum CompileError {
    IO(std::io::Error),
    Glium(glium::ProgramCreationError)
}

/*
fn try_compile_program<P: AsRef<Path>>(display: &GlutinFacade, path: P) -> Result<Program, CompileError> {
    let mut fragment_shader_script = String::new();

    if let Err(err) = File::open(path).and_then(|mut file| file.read_to_string(&mut fragment_shader_script)) {
        return Err(CompileError::IO(err))
    }

    let fragment_shader_source = &shady_script::parse(&fragment_shader_script).generate_fragment_shader();
    println!("{}", fragment_shader_source);

    glium::Program::from_source(display, vertex_shader_source, &fragment_shader_source, None)
        .map_err(|err| CompileError::Glium(err))
}
*/

fn load_images<P: AsRef<Path>>(displays: &mut Vec<ImageDisplay>, path: P) {
    let mut idx = 0usize;

    shady_script::parse_file(path).analyse().with_images(|image| {
        let shader = image.standalone_shader();
        println!("Generated Shader: {}\n", shader);

        let new_display = match displays.get_mut(idx) {
            Some(mut display) => {
                display.program = Program::from_source(&display.display, vertex_shader_source, &shader, None).unwrap();
                None
            }

            None => {
                let display = glium::glutin::WindowBuilder::new()
                    .with_title(format!("Shady Image {}", idx))
                    .with_dimensions(500, 500)
                    .build_glium()
                    .unwrap();

                let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
                let program = Program::from_source(&display, vertex_shader_source, &shader, None).unwrap();

                Some(ImageDisplay {
                    display: display,
                    buffer: vertex_buffer,
                    program: program,
                    done: false
                })
            }
        };

        if let Some(display) = new_display {
            displays.push(display)
        }

        idx += 1;
    });
}

fn main() {
    let path = Path::new("script.shy");

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    let mut displays = Vec::new();
    load_images(&mut displays, path);

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx).unwrap();
    watcher.watch(path).unwrap();

    loop {
        match rx.try_recv() {
            Ok(_) => load_images(&mut displays, path),
            Err(_) => (),
        };

        for display in &mut displays {
            let mut done = false;

            for event in display.display.poll_events() {
                match event {
                    glium::glutin::Event::Closed => done = true,
                    _ => ()
                }
            }

            display.done = done;

            let mut target = display.display.draw();
            target.clear_color(1.0, 0.0, 0.0, 0.0);
            target.draw(&display.buffer, &indices, &display.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            target.finish().unwrap();
        }

        displays.retain(|display| display.done == false);
        if displays.is_empty() {
            break
        }
    }
}
