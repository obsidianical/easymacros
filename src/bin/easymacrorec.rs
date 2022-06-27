use std::os::raw::{c_char, c_uint};
use std::process::{exit, ExitCode};
use clap::Parser;
use x11::keysym::XK_Escape;
use x11::xinput2::XIGrabModeSync;
use x11::xlib::{CurrentTime, GrabModeAsync, GrabModeSync, GrabSuccess, KeyCode, KeyPress, KeyPressMask, SyncPointer, XEvent, XPointer};
use x11::xrecord::{XRecordEndOfData, XRecordInterceptData, XRecordStartOfData};
use easymacros::x11_safe_wrapper::{Keycode, XDisplay};

/// Macro recording module for easymacros. Outputs are partially compatible with xmacro.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// The file that contains the macro to run.
	#[clap(value_parser, value_name = "output_file", value_hint = clap::ValueHint::FilePath)]
	output_file: Option<std::path::PathBuf>,
	/// Display to run the macro on. This uses the $DISPLAY environment variable by default.
	#[clap(short, long)]
	display: Option<String>,
}
fn main() {
	let args = Args::parse();

	let display = XDisplay::open(args.display);

	let stop_key = get_stop_key(display);

	dbg!(stop_key);
}

fn get_stop_key(display: XDisplay) -> Keycode {
	let screen = display.get_default_screen();

	let root = display.get_root_window(screen);
	let potential_err = display.grab_keyboard(root, false, GrabModeSync, GrabModeAsync, CurrentTime);

	if potential_err != GrabSuccess {
		eprintln!("Couldn't grab keyboard!");
		exit(1);
	}

	println!("Press the key you want to use to stop recording the macro.");

	let mut stop_key: Keycode = XK_Escape;

	loop {
		display.allow_events(SyncPointer, CurrentTime);
		let ev = display.window_event(root, KeyPressMask);

		unsafe {
			match ev {
				XEvent { key } => {
					stop_key = key.keycode;
					break;
				}
				_ => {},
			}
		}
	}

	stop_key
}

fn ev_loop(display: XDisplay, screen: i32, stop_key: Keycode) {
	let root = display.get_root_window(screen);
}

struct EvCallbackData {
	stop_key: Keycode,
	x: i32,
	y: i32,
}

unsafe extern "C" fn ev_callback(closure: *mut c_char, intercept_data: *mut XRecordInterceptData) {
	let data = &mut *(closure as *mut EvCallbackData);
	let intercept_data = &mut *intercept_data;

	if intercept_data.category == XRecordStartOfData { println!("Got start of data!"); }
	else if intercept_data.category == XRecordEndOfData { println!("Got end of data!");}


}