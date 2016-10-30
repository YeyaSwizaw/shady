pub use self::platform::*;

#[cfg(target_os="macos")]
#[path="macos.rs"]
mod platform;

#[cfg(all(not(target_os="macos")))]
mod platform {
    use std::path::Path;

    pub fn save_image<F: FnMut(Path)>(_: F) {}
}
