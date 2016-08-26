#[macro_use] extern crate glium;
extern crate clap;
extern crate notify;
extern crate shady_script;

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::sync::mpsc::channel;

use glium::{Program, VertexBuffer, DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;

use clap::{App, Arg};

use notify::{RecommendedWatcher, Watcher};

use shady_script::{ParseError, AnalyseError};

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

    loop {
        if let Some((ref rx, _)) = watcher {
            if let Ok(_) = rx.try_recv() {
                if let Err(err) = load_images(&mut buffer, &mut displays, path) {
                    println!("{:?}", err);
                }
            };
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

        displays.retain(|display| !display.done);
        if displays.is_empty() && !keep {
            break
        }
    }
}
