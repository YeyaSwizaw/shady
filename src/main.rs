#![feature(slice_patterns)]

#[macro_use] extern crate glium;
extern crate clap;
extern crate notify;
extern crate shady_script;

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::sync::mpsc::channel;
use std::time::Instant;

use glium::{Program, VertexBuffer, DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::uniforms::{EmptyUniforms, Uniforms};

use clap::{App, Arg};

use notify::{RecommendedWatcher, Watcher};

use shady_script::{ParseError, AnalyseError, Uniform};

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
    uniforms: Vec<Uniform>,
    mouse_position: (f32, f32),
    done: bool
}

#[derive(Debug)]
enum Error<'a> {
    IO(std::io::Error),
    Parse(ParseError<'a>),
    Analyse(AnalyseError),
}

fn load_images<'a, P: AsRef<Path>>(buffer: &'a mut String, displays: &mut Vec<ImageDisplay>, path: P) -> Result<(), Error<'a>> {
    buffer.clear();

    let mut idx = 0usize;

    if let Err(err) = File::open(path).and_then(|mut file| file.read_to_string(buffer)) {
        return Err(Error::IO(err))
    }

    let ast = match shady_script::parse_input(buffer) {
        Ok(ast) => ast,
        Err(err) => return Err(Error::Parse(err))
    };

    let sdy = match ast.analyse() {
        Ok(sdy) => sdy,
        Err(err) => return Err(Error::Analyse(err))
    };

    sdy.with_images(|image| {
        let shader = image.standalone_shader();
        println!("\nGenerated Shader {}:\n{}\n", idx, shader);

        let new_display = match displays.get_mut(idx) {
            Some(mut display) => {
                display.display.get_window().unwrap().set_title(&format!("Shady Image {}", idx));
                display.program = Program::from_source(&display.display, vertex_shader_source, &shader, None).unwrap();
                display.uniforms = image.standalone_uniforms();
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
                    uniforms: image.standalone_uniforms(),
                    mouse_position: (0.0, 0.0),
                    done: false
                })
            }
        };

        if let Some(display) = new_display {
            displays.push(display)
        }

        idx += 1;
    });

    Ok(())
}

fn main() {
    let matches = App::new("Shady")
        .author("Samuel Sleight <samuel.sleight@gmail.com>")
        .version("0.1.0")
        .arg(Arg::with_name("script")
             .help("The script to load images from")
             .required(true))
        .arg(Arg::with_name("once")
             .help("Only load images once; do not watch the script for changes")
             .long("once")
             .short("o"))
        .arg(Arg::with_name("keep")
             .help("Keep watching the script if all windows are closed")
             .long("keep")
             .short("k"))
        .get_matches();

    let path = Path::new(matches.value_of("script").unwrap());
    let once = matches.is_present("once");
    let keep = !once && matches.is_present("keep");

    let mut buffer = String::new();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    let mut displays = Vec::new();
    if let Err(err) = load_images(&mut buffer, &mut displays, path) {
        println!("{:?}", err);
    }

    let watcher = if once {
        None
    } else {
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx).unwrap();
        watcher.watch(path).unwrap();
        Some((rx, watcher))
    };

    let mut time = Instant::now();

    loop {
        if let Some((ref rx, _)) = watcher {
            if let Ok(_) = rx.try_recv() {
                time = Instant::now();

                if let Err(err) = load_images(&mut buffer, &mut displays, path) {
                    println!("{:?}", err);
                }
            };
        };

        let duration = time.elapsed().subsec_nanos() as f32 / 1000000000.0;

        for display in &mut displays {
            let size = display.display.get_window().unwrap().get_inner_size_pixels().unwrap();

            for event in display.display.poll_events() {
                match event {
                    glium::glutin::Event::Closed => display.done = true,
                    glium::glutin::Event::MouseMoved(x, y) => display.mouse_position = (x as f32 / size.0 as f32, y as f32 / size.1 as f32),
                    _ => ()
                }
            }

            let mut target = display.display.draw();
            target.clear_color(1.0, 0.0, 0.0, 0.0);

            macro_rules! render {
                ($uniforms:expr) => (target.draw(&display.buffer, &indices, &display.program, &$uniforms, &Default::default()).unwrap())
            };

            match display.uniforms.as_slice() {
                &[] => render!(EmptyUniforms),

                &[Uniform::Time] => render!(uniform! {
                    time: duration
                }),

                &[Uniform::Time, Uniform::MouseX] => render!(uniform! {
                    time: duration,
                    mouse_x: display.mouse_position.0,
                }),

                &[Uniform::Time, Uniform::MouseY] => render!(uniform! {
                    time: duration,
                    mouse_y: display.mouse_position.1,
                }),

                &[Uniform::MouseX, Uniform::MouseY] => render!(uniform! {
                    mouse_x: display.mouse_position.0,
                    mouse_y: display.mouse_position.1,
                }),

                &[Uniform::Time, Uniform::MouseX, Uniform::MouseY] => render!(uniform! {
                    time: duration,
                    mouse_x: display.mouse_position.0,
                    mouse_y: display.mouse_position.1,
                }),

                _ => panic!("Unexpected uniform format - this shouldn't happen")
            };

            target.finish().unwrap();
        }

        displays.retain(|display| !display.done);
        if displays.is_empty() && !keep {
            break
        }
    }
}
