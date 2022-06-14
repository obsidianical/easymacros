use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint};
use std::process::exit;
use std::slice::from_raw_parts;
use std::thread;
use std::time::Duration;
use x11::xlib::{Display, XCloseDisplay, XDisplayString, XFlush, XKeysymToKeycode, XOpenDisplay, XStringToKeysym, XSync};
use easymacros::add;
use clap::Parser;
use x11::keysym::{XK_d, XK_Super_L};
use x11::xtest::{XTestFakeButtonEvent, XTestFakeKeyEvent, XTestFakeMotionEvent, XTestGrabControl, XTestQueryExtension};
use x11_keysymdef::lookup_by_name;

/// Macro program inspired by xmacro.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Display
    #[clap(short, long)]
    display: String,
}

fn main () {
    let args = Args::parse();

    let display = get_remote(args.display);

    let super_l_str_ptr = b"Super_L\0".as_ptr();
    let super_l_sym = unsafe { XStringToKeysym(super_l_str_ptr as *const i8) };
    let super_l_code = unsafe { XKeysymToKeycode(display, super_l_sym) };

    let d_str_ptr = b"d\0".as_ptr();
    let d_sym = unsafe { XStringToKeysym(d_str_ptr as *const i8) };
    let d_code = unsafe { XKeysymToKeycode(display, d_sym) };

    unsafe {
        XTestFakeKeyEvent(display, super_l_code as c_uint, c_int::from(true), 10);
        XFlush(display);
        XTestFakeKeyEvent(display, d_code as c_uint, c_int::from(true), 10);
        XFlush(display);

        XTestFakeKeyEvent(display, d_code as c_uint, c_int::from(false), 10);
        XFlush(display);
        XTestFakeKeyEvent(display, super_l_code as c_uint, c_int::from(false), 10);
        XFlush(display);

        XCloseDisplay(display);
    }

    println!("play: {}", add(5, 10));
}

fn get_remote(dpy_name: String) -> *mut Display {
    let dpy_name = CString::new(dpy_name).unwrap();
    let display_ptr: *const u8 = dpy_name.as_bytes().as_ptr();
    let display = unsafe { XOpenDisplay(display_ptr as *const i8) };

    let mut ev = c_int::default();
    let mut err = c_int::default();
    let mut version = (c_int::default(), c_int::default());

    if unsafe { XTestQueryExtension(display, &mut ev, &mut err, &mut version.0, &mut version.1) } == 0 {
        eprintln!("XTest not supported!");
        unsafe { XCloseDisplay(display) };
        exit(1)
    }

    unsafe {
        XTestGrabControl(display, c_int::from(true));
        XSync(display, c_int::from(false));
    };

    display
}
