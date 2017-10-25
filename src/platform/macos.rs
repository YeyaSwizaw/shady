use std::ffi::CStr;
use std::path::Path;

use glium::glutin::{EventsLoop, ContextBuilder, WindowBuilder};
use glium::backend::glutin::Display;
use glium::glutin::os::macos::{ActivationPolicy, WindowBuilderExt, WindowExt};

use cocoa::base::id;
use cocoa::foundation::{NSString, NSArray, NSUInteger};
use cocoa::appkit::{NSApp, NSApplication, NSMenu, NSWindow};
use objc::runtime::{Class, NO};

pub fn open_window(event_loop: &EventsLoop, title: &str, (w, h): (u32, u32)) -> Display {
    let display = Display::new(
        WindowBuilder::new()
            .with_title(title)
            .with_dimensions(w, h),
            // .with_activation_policy(ActivationPolicy::Accessory)
        ContextBuilder::new(),
        event_loop
    )
        .unwrap();

    unsafe {
        let app = (display.gl_window().window().get_nswindow() as id).windowController();
        
        let title = NSString::alloc(0 as id).init_str("Shady");
        let menu = NSMenu::alloc(0 as id).initWithTitle_(title);
        // app.setMainMenu_(menu);
    }

    display
}

pub fn save_image<F: FnMut(&Path)>(mut f: F) {
    unsafe {
        let ns_savepanel = Class::get("NSSavePanel").unwrap();
        let savepanel: id = msg_send![ns_savepanel, savePanel];
        let _: () = msg_send![savepanel, setExtensionHidden:NO];

        let filetype = NSString::alloc(0 as id).init_str("png");
        let _: () = msg_send![savepanel, setAllowedFileTypes:NSArray::arrayWithObject(0 as id, filetype)];

        let result: NSUInteger = msg_send![savepanel, runModal];
        if result == 1 { // ok
            let filename: id = msg_send![savepanel, filename];
            let path = CStr::from_ptr(filename.UTF8String()).to_string_lossy().into_owned();
            f(&Path::new(&path));
        }
    }
}
