use std::env;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint, c_ulong};
use x11::xlib::{
    Display, XCloseDisplay, XFlush, XKeysymToKeycode, XOpenDisplay, XStringToKeysym, XSync,
};
use x11::xtest::{
    XTestFakeButtonEvent, XTestFakeKeyEvent, XTestFakeMotionEvent, XTestGrabControl,
    XTestQueryExtension,
};

pub struct XDisplay {
    ptr: *mut Display,
}

pub type Keysym = c_ulong;
pub type Keycode = c_uint;

const FALSE_C: c_int = 0;
const TRUE_C: c_int = 1;

impl XDisplay {
    pub fn open(display_name: Option<String>) -> Self {
        let name = CString::new(if let Some(name) = display_name {
            name
        } else {
            env::var("DISPLAY").expect("DISPLAY is not set")
        })
        .unwrap();
        let name_ptr = name.as_bytes().as_ptr();
        let display_ptr = unsafe { XOpenDisplay(name_ptr as *const i8) };

        Self { ptr: display_ptr }
    }

    pub fn close(self) {
        unsafe { XCloseDisplay(self.ptr) };
    }

    pub fn sync(&self) {
        unsafe {
            XSync(self.ptr, c_int::from(false));
        }
    }

    pub fn flush(&self) {
        unsafe {
            XFlush(self.ptr);
        }
    }

    pub fn keysym_to_keycode(&self, keysym: c_ulong) -> Keycode {
        unsafe { XKeysymToKeycode(self.ptr, keysym) as Keycode }
    }

    pub fn string_to_keycode(&self, string: &[u8]) -> Keycode {
        self.keysym_to_keycode(string_to_keysym(string))
    }

    pub fn has_xtest(&self) -> bool {
        let mut vals: (c_int, c_int, c_int, c_int) = (0, 0, 0, 0);
        let has_extension = unsafe {
            XTestQueryExtension(self.ptr, &mut vals.0, &mut vals.1, &mut vals.2, &mut vals.3)
        };
        has_extension != 0
    }

    pub fn send_fake_keypress_from_string(&self, string: &[u8]) {
        self.send_fake_keypress_from_keysym(string_to_keysym(string))
    }

    pub fn send_fake_keypress_from_keysym(&self, ks: Keysym) {
        self.send_fake_keypress_from_code(self.keysym_to_keycode(ks))
    }

    pub fn send_fake_keypress_from_code(&self, code: Keycode) {
        unsafe { XTestFakeKeyEvent(self.ptr, code, TRUE_C, 10) };
        self.flush();
    }

    pub fn send_fake_buttonpress(&self, button: u32) {
        unsafe { XTestFakeButtonEvent(self.ptr, button, TRUE_C, 10) };
    }

    pub fn send_fake_buttonrelease(&self, button: u32) {
        unsafe { XTestFakeButtonEvent(self.ptr, button, FALSE_C, 10) };
    }

    pub fn send_fake_keyrelease_from_string(&self, string: &[u8]) {
        self.send_fake_keyrelease_from_keysym(string_to_keysym(string))
    }

    pub fn send_fake_keyrelease_from_keysym(&self, ks: Keysym) {
        self.send_fake_keyrelease_from_code(self.keysym_to_keycode(ks))
    }

    pub fn send_fake_keyrelease_from_code(&self, code: Keycode) {
        unsafe { XTestFakeKeyEvent(self.ptr, code, FALSE_C, 10) };
        self.flush();
    }

    pub fn send_fake_motion_event(&self, x: c_int, y: c_int) {
        unsafe { XTestFakeMotionEvent(self.ptr, -1, x, y, 10) };
        self.flush();
    }

    pub fn grab_control(&self) {
        unsafe {
            XTestGrabControl(self.ptr, TRUE_C);
        }
    }
}

/// Wrapper for XStringToKeysym. Remember to give a null terminated string!
pub fn string_to_keysym(string: &[u8]) -> Keysym {
    unsafe { XStringToKeysym(string.as_ptr() as *const c_char) }
}
