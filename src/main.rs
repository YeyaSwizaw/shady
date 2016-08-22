#[macro_use] extern crate glium;
extern crate notify;

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::sync::mpsc::channel;

use glium::Program;
use glium::{DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;

use notify::{RecommendedWatcher, Watcher};

#[derive(Copy, Clone)]
struct Vertex {
    v_xy: [f32; 2],
    v_uv: [f32; 2]
}

implement_vertex!(Vertex, v_xy, v_uv);

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

#[derive(Debug)]
enum CompileError {
    IO(std::io::Error),
    Glium(glium::ProgramCreationError)
}

fn try_compile_program<P: AsRef<Path>>(display: &GlutinFacade, path: P) -> Result<Program, CompileError> {
    let mut fragment_shader_source = String::new();

    if let Err(err) = File::open(path).and_then(|mut file| file.read_to_string(&mut fragment_shader_source)) {
        return Err(CompileError::IO(err))
    }

    glium::Program::from_source(display, vertex_shader_source, &fragment_shader_source, None)
        .map_err(|err| CompileError::Glium(err))
}

fn main() {
    let path = Path::new("shader.glsl");

    let display = glium::glutin::WindowBuilder::new()
        .with_title("Shady")
        .with_dimensions(400, 400)
        .build_glium()
        .unwrap();

    let shape = [
        Vertex { v_xy: [-1.0, -1.0], v_uv: [0.0, 0.0] },
        Vertex { v_xy: [ 1.0, -1.0], v_uv: [1.0, 0.0] },
        Vertex { v_xy: [ 1.0,  1.0], v_uv: [1.0, 1.0] },
        Vertex { v_xy: [-1.0,  1.0], v_uv: [0.0, 1.0] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    let mut program = try_compile_program(&display, path).unwrap();

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx).unwrap();
    watcher.watch(path).unwrap();

    'main_loop: loop {
        match rx.try_recv() {
            Ok(_) => program = try_compile_program(&display, path).unwrap(),
            Err(_) => (),
        };

        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => break 'main_loop,
                _ => ()
            }
        }

        let mut target = display.draw();
        target.clear_color(1.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    }
}
