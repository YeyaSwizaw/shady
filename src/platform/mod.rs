use glium::backend::glutin_backend::GlutinFacade;

pub use self::platform::*;

#[cfg(target_os="macos")]
#[path="macos.rs"]
mod platform;

#[cfg(all(not(target_os="macos")))]
mod platform {
    pub fn init_platform_window(_: &GlutinFacade) {}

    pub fn platform_drag(_: &GlutinFacade, _: (i32, i32)) {}
}

