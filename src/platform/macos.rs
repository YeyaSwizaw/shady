use std::ffi::CStr;
use std::path::Path;

use cocoa::base::id;
use cocoa::foundation::{NSString, NSArray, NSUInteger};
use objc::runtime::{Object, Class, NO};

pub fn save_image<F: FnMut(&Path)>(mut f: F) {
    unsafe {
        let ns_savepanel = Class::get("NSSavePanel").unwrap();
        let savepanel: id = msg_send![ns_savepanel, savePanel];
        let _: () = msg_send![savepanel, setExtensionHidden:NO];

        let filetype = NSString::alloc(0 as id).init_str("png");
        let _: () = msg_send![savepanel, setAllowedFileTypes:NSArray::arrayWithObject(0 as id, filetype)];

        let result: NSUInteger = msg_send![savepanel, runModal];
        if result == 1 {
            let filename: id = msg_send![savepanel, filename];
            let path = CStr::from_ptr(filename.UTF8String()).to_string_lossy().into_owned();
            f(&Path::new(&path));
        }
    }
}
