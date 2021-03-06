#![feature(type_ascription, slice_patterns)]

#[macro_use] extern crate glium;
extern crate clap;
extern crate notify;
extern crate shady_script;
extern crate imagefmt;

#[cfg(target_os="macos")]
extern crate cocoa;

#[cfg(target_os="macos")]
#[macro_use] extern crate objc;

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::sync::mpsc::channel;
use std::time::Instant;

use imagefmt::{ColType, ColFmt, png};

use glium::{Program, VertexBuffer, Surface};
use glium::glutin::{WindowId, EventsLoop, Event, WindowEvent};
use glium::texture::RawImage2d;
use glium::texture::texture2d::Texture2d;
use glium::backend::glutin::Display;
use glium::uniforms::EmptyUniforms;

use clap::{App, Arg};

use notify::{RecommendedWatcher, Watcher, RecursiveMode};

use shady_script::{ParseError, AnalyseError, Uniform};

mod platform;

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
    display: Display,
    id: WindowId,
    buffer: VertexBuffer<Vertex>,
    program: Program,
    uniforms: Vec<Uniform>,
    mouse_position: (i32, i32),
    done: bool,
}

#[derive(Debug)]
enum Error<'a> {
    IO(std::io::Error),
    Parse(ParseError<'a>),
    Analyse(AnalyseError),
}

fn load_images<'a, P: AsRef<Path>>(buffer: &'a mut String, event_loop: &EventsLoop, displays: &mut Vec<ImageDisplay>, path: P) -> Result<(), Error<'a>> {
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
                display.display.gl_window().window().set_title(&format!("Shady Image {}", idx));
                display.program = Program::from_source(&display.display, vertex_shader_source, &shader, None).unwrap();
                display.uniforms = image.standalone_uniforms();
                None
            }

            None => {
                let display = platform::open_window(event_loop, &format!("Shady Image {}", idx), (500, 500));
                let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
                let program = Program::from_source(&display, vertex_shader_source, &shader, None).unwrap();

                let id = display.gl_window().window().id();

                Some(ImageDisplay {
                    display: display,
                    id: id,
                    buffer: vertex_buffer,
                    program: program,
                    uniforms: image.standalone_uniforms(),
                    mouse_position: (0, 0),
                    done: false,
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

fn with_display<F: FnMut(&mut ImageDisplay)>(displays: &mut [ImageDisplay], id: WindowId, mut f: F) {
    for display in displays {
        if display.id == id {
            f(display);
            return
        }
    }
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
    let mut displays = Vec::new();
    let mut event_loop = EventsLoop::new();

    if let Err(err) = load_images(&mut buffer, &event_loop, &mut displays, path) {
        println!("{:?}", err);
    }

    let watcher = if once {
        None
    } else {
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new_raw(tx).unwrap();
        watcher.watch(path, RecursiveMode::Recursive).unwrap();
        Some((rx, watcher))
    };

    let mut time = Instant::now();
    loop {
        if let Some((ref rx, _)) = watcher {
            if let Ok(_) = rx.try_recv() {
                time = Instant::now();

                if let Err(err) = load_images(&mut buffer, &event_loop, &mut displays, path) {
                    println!("{:?}", err);
                }
            };
        };

        let duration = time.elapsed().subsec_nanos() as f32 / 1000000000.0;

        event_loop.poll_events(|event| {
            if let Event::WindowEvent { event: e, window_id: id } = event {
                match e {
                    WindowEvent::Closed => with_display(&mut displays, id, |display| display.done = true),
                    WindowEvent::MouseMoved { position, .. } => with_display(&mut displays, id, |display| display.mouse_position = (position.0 as i32, position.1 as i32)),

                    _ => ()
                }
            }
        });

        for display in &mut displays {
            /*
            if save {
                platform::save_image(|path| {
                    let tex = Texture2d::empty(&display.display, size.0, size.1).unwrap();
                    let mut target = tex.as_surface();
                    render(
                        &mut target, 
                        &display.program, 
                        &display.buffer, 
                        &display.uniforms, 
                        duration, 
                        display.mouse_position.0 as f32 / size.0 as f32, 
                        display.mouse_position.1 as f32 / size.1 as f32
                    );

                    let raw: RawImage2d<u8> = tex.read();
                    let mut file = File::create(path).unwrap();
                    png::write(&mut file, raw.width as usize, raw.height as usize, ColFmt::RGBA, &raw.data, ColType::Auto, None).unwrap();
                });
            }
            */

            let size = display.display.gl_window().window().get_inner_size_pixels().unwrap();
            let mut target = display.display.draw();

            render(
                &mut target, 
                &display.program, 
                &display.buffer, 
                &display.uniforms, 
                duration, 
                display.mouse_position.0 as f32 / size.0 as f32, 
                display.mouse_position.1 as f32 / size.1 as f32
            );

            target.finish().unwrap();

        }

        displays.retain(|display| !display.done);
        if displays.is_empty() && !keep {
            break
        }
    }
}

fn render<S: Surface>(surface: &mut S, program: &Program, buffer: &VertexBuffer<Vertex>, uniforms: &[Uniform], time: f32, mx: f32, my: f32) {
    surface.clear_color(0.0, 0.0, 0.0, 0.0);

    macro_rules! render {
        ($uniforms:expr) => (surface.draw(
            buffer, 
            &glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan), 
            program, 
            &$uniforms, 
            &Default::default()
        ).unwrap())
    };

    match uniforms {
        &[] => render!(EmptyUniforms),

        &[Uniform::Time] => render!(uniform! {
            time: time
        }),

        &[Uniform::MouseX] => render!(uniform! {
            mouse_x: mx
        }),

        &[Uniform::MouseY] => render!(uniform! {
            mouse_y: my
        }),

        &[Uniform::Time, Uniform::MouseX] => render!(uniform! {
            time: time,
            mouse_x: mx
        }),

        &[Uniform::Time, Uniform::MouseY] => render!(uniform! {
            time: time,
            mouse_y: my
        }),

        &[Uniform::MouseX, Uniform::MouseY] => render!(uniform! {
            mouse_x: mx,
            mouse_y: my
        }),

        &[Uniform::Time, Uniform::MouseX, Uniform::MouseY] => render!(uniform! {
            time: time,
            mouse_x: mx,
            mouse_y: my,
        }),

        _ => panic!("Unexpected uniform format - this shouldn't happen")
    };
}
