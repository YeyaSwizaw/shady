use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::os::macos::WindowExt;
use glium::texture::RawImage2d;
use glium::texture::texture2d::Texture2d;

use objc::runtime::{Object, Class};

use imagefmt::{ColType, ColFmt, png};

use std::borrow::Borrow;
use std::ffi::CStr;
use std::fs::File;
use std::os::raw::c_char;

pub fn init_platform_window(display: &GlutinFacade) {
    /*
    let wnd = display.get_window().unwrap().get_nswindow() as *mut Object;

    unsafe {
        let title: *mut Object = msg_send![wnd, title];

        let ns_toolbar = Class::get("NSToolbar").unwrap();
        let toolbar: *mut Object = msg_send![ns_toolbar, alloc];
        let toolbar: *mut Object = msg_send![toolbar, initWithIdentifier:title];

        let _: () = msg_send![wnd, setToolbar:toolbar];
        let _: () = msg_send![toolbar, release];
    }
    */
}

pub fn platform_drag(texture: &Texture2d, (mx, my): (i32, i32)) {
    println!("Starting drag at: {}, {}", mx, my);

    let image: RawImage2d<u8> = texture.read();

    //let mut buffer = Vec::new();
    let mut buffer = File::create("test.png").unwrap();
    png::write(&mut buffer, image.width as usize, image.height as usize, ColFmt::RGBA, &image.data, ColType::Auto, None).unwrap();
}
