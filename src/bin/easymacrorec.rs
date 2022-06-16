use std::os::raw::c_uint;
use std::process::{exit, ExitCode};
use clap::Parser;
use x11::keysym::XK_Escape;
use x11::xinput2::XIGrabModeSync;
use x11::xlib::{CurrentTime, GrabModeAsync, GrabModeSync, GrabSuccess, KeyCode, KeyPress, KeyPressMask, SyncPointer, XEvent};
use easymacros::x11_safe_wrapper::{Keycode, XDisplay};

/// Macro recording module for easymacros. Outputs are partially compatible with xmacro.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// The file that contains the macro to run.
	#[clap(value_parser, value_name = "input_file", value_hint = clap::ValueHint::FilePath)]
	output_file: std::path::PathBuf,
	/// Display to run the macro on. This uses the $DISPLAY environment variable by default.
	#[clap(short, long)]
	display: Option<String>,
}
fn main() {
	let args = Args::parse();

	let display = XDisplay::open(args.display);

	let stop_key = get_stop_key(display);


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

	let mut stop_key = XK_Escape;

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

unsafe extern "C" fn ev_callback() {

}