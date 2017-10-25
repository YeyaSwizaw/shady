pub use self::platform::*;

#[cfg(target_os="macos")]
#[path="macos.rs"]
mod platform;

#[cfg(all(not(target_os="macos")))]
mod platform {
    use std::path::Path;

    use glium::glutin::{EventsLoop, ContextBuilder, WindowBuilder};
    use glium::backend::glutin::Display;

    pub fn open_window(event_loop: &EventsLoop, title: &str, (w, h): (u32, u32)) -> Display {
        Display::new(
            glium::glutin::WindowBuilder::new()
                .with_title(title)
                .with_dimensions(w, h),
            ContextBuilder::new(),
            event_loop
        )
            .unwrap();

    }

    pub fn save_image<F: FnMut(Path)>(_: F) {}
}
