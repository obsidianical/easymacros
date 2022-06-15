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
use easymacros::x11_safe_wrapper::XDisplay;

/// Macro program inspired by xmacro.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Display
    #[clap(short, long)]
    display: String,
    /// xmacro compatibility
    #[clap(long)]
    xmacro: bool,
}

fn main () {
    let args = Args::parse();
    let xmacro_mode = args.xmacro;

    let display = get_remote(None);

    display.send_fake_keypress_from_string(b"Super_L\0");
    display.send_fake_keypress_from_string(b"d\0");
    display.send_fake_keyrelease_from_string(b"d\0");
    display.send_fake_keyrelease_from_string(b"Super_L\0");

    display.close();
}

fn get_remote(display_name: Option<String>) -> XDisplay {
    let display = XDisplay::open(display_name);

    if !display.has_xtest() {
        eprintln!("XTest not supported!");
        display.close();
        exit(1)
    }

    display.grab_control();
    display.sync();

    display
}
