use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint};
use std::process::exit;
use std::slice::from_raw_parts;
use std::{fs, thread};
use std::time::Duration;
use x11::xlib::{Display, XCloseDisplay, XDisplayString, XFlush, XKeysymToKeycode, XOpenDisplay, XStringToKeysym, XSync};
use clap::Parser;
use x11::keysym::{XK_d, XK_Super_L};
use x11::xtest::{XTestFakeButtonEvent, XTestFakeKeyEvent, XTestFakeMotionEvent, XTestGrabControl, XTestQueryExtension};
use x11_keysymdef::lookup_by_name;
use easymacros::x11_safe_wrapper::{Keysym, XDisplay};



/// Macro program inspired by xmacro.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[clap(value_parser, value_name = "input_file", value_hint = clap::ValueHint::FilePath)]
    input_file: std::path::PathBuf,
    /// Display
    #[clap(short, long)]
    display: Option<String>,
    // xmacro compatibility, currently the only supported input format anyway
    // #[clap(long)]
    // xmacro: bool,
}

fn main () {
    let args = Args::parse();
    // let xmacro_mode = args.xmacro;

    let input_file_contents = fs::read_to_string(args.input_file).expect("couldn't read macro file");
    let display = get_remote(args.display);

    for instruction in input_file_contents.lines() {
        println!("Instruction: {}", instruction);
        let command: Vec<&str> = instruction.split(' ').collect();

        match command[0] {
            "Delay" => thread::sleep(Duration::from_millis(command[1].parse().unwrap())),
            "ButtonPress" => display.send_fake_buttonpress(command[1].parse().unwrap()),
            "ButtonRelease" => display.send_fake_buttonrelease(command[1].parse().unwrap()),
            "MotionNotify" => display.send_fake_motion_event(command[1].parse().unwrap(), command[2].parse().unwrap()),
            "KeyCodePress" => display.send_fake_keypress_from_code(command[1].parse().unwrap()),
            "KeyCodeRelease" => display.send_fake_keyrelease_from_code(command[1].parse().unwrap()),
            "KeySymPress" => display.send_fake_keypress_from_keysym(command[1].parse().unwrap()),
            "KeySymRelease" => display.send_fake_keyrelease_from_keysym(command[1].parse().unwrap()),
            "KeySym" => {
                let key: Keysym = command[1].parse().unwrap();
                display.send_fake_keypress_from_keysym(key);
                display.send_fake_keyrelease_from_keysym(key);
            },
            "KeyStrPress" => display.send_fake_keypress_from_string(CString::new(command[1]).unwrap().as_bytes()),
            "KeyStrRelease" => display.send_fake_keyrelease_from_string(CString::new(command[1]).unwrap().as_bytes()),
            "KeyStr" => {
                let keystring = CString::new(command[1]).unwrap();
                display.send_fake_keypress_from_string(keystring.as_bytes());
                display.send_fake_keyrelease_from_string(keystring.as_bytes());
            },
            "String" => {
                println!("Strings are currently not supported.");
                // for c in instruction[7..].chars() {
                //     display.send_fake_keypress_from_string(CString::new(c.to_string()).unwrap().as_bytes());
                //     display.send_fake_keyrelease_from_string(CString::new(c.to_string()).unwrap().as_bytes());
                // }
            }
            c => {panic!("Unknown command {}", c)}
        }
    }

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
