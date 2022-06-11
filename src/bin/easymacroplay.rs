use std::ffi::CString;
use std::os::raw::c_int;
use x11::xlib::{XCloseDisplay, XFlush, XOpenDisplay};
use easymacros::add;
use clap::Parser;
use x11::keysym::{XK_d, XK_Super_L};
use x11::xtest::{XTestFakeButtonEvent, XTestFakeKeyEvent, XTestFakeMotionEvent, XTestGrabControl};
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

    // let display_ptr = args.display.as_bytes().as_ptr() as *const i8;
    println!("Display name: {}", args.display);
    let display_name = CString::new(args.display).unwrap();
    println!("Display name cstr: {:?}", display_name);
    let display_ptr: *const u8 = display_name.as_bytes().as_ptr();
    println!("Display name ptr: {:?}", display_ptr);
    let display = unsafe { XOpenDisplay(display_ptr as *const i8) };
    println!("Display name ptr: {:?}", display);

    let super_l = lookup_by_name("Super_L").unwrap();
    let d = lookup_by_name("d").unwrap();

    unsafe {
        // XTestFakeKeyEvent(display, super_l.keysym, 1, 0);
        // XTestFakeKeyEvent(display, d.keysym, 1, 10);
        //
        // XTestFakeKeyEvent(display, d.keysym, 0, 20);
        // XTestFakeKeyEvent(display, super_l.keysym, 0, 30);
        XTestGrabControl(display, c_int::from(false));
        XTestFakeKeyEvent(display, XK_Super_L, c_int::from(true), 0);
        XFlush(display);
        XTestFakeKeyEvent(display, XK_d, c_int::from(true), 0);
        XFlush(display);
        XTestFakeKeyEvent(display, XK_d, c_int::from(false), 0);
        XFlush(display);
        XTestFakeKeyEvent(display, XK_Super_L, c_int::from(false), 0);
        XFlush(display);

        XTestFakeMotionEvent(display, -1, 200, 200, 0);
        XFlush(display);

        XCloseDisplay(display);
    }


    println!("play: {}", add(5, 10));
}

