extern crate core;

use std::os::raw::{c_char, c_uchar, c_uint};
use std::process::{exit, ExitCode};
use std::ptr::{addr_of, slice_from_raw_parts};
use std::{slice, thread};
use std::ffi::c_void;
use clap::Parser;
use x11::keysym::XK_Escape;
use x11::xinput2::XIGrabModeSync;
use x11::xlib::{ButtonPress, ButtonRelease, CurrentTime, GrabModeAsync, GrabModeSync, GrabSuccess, KeyCode, KeyPress, KeyPressMask, KeyRelease, MotionNotify, SyncPointer, XEvent, XFree, XKeyEvent, XKeyPressedEvent, XPointer};
use x11::xrecord::{XRecordAllocRange, XRecordContext, XRecordCreateContext, XRecordDisableContext, XRecordEndOfData, XRecordFreeData, XRecordInterceptData, XRecordStartOfData};
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

	let display = XDisplay::open(args.display.clone());
	let recorded_display = XDisplay::open(args.display.clone());

	let stop_key = get_stop_key(display);

	let screen = display.get_default_screen();
	dbg!(stop_key);

	ev_loop(display, recorded_display, screen, stop_key);
	display.close();
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

fn ev_loop(display: XDisplay, recordeddpy: XDisplay, screen: i32, stop_key: Keycode) {
	let root = display.get_root_window(screen);
	let protocol_ranges = unsafe { XRecordAllocRange() };

	let ctx = recordeddpy.create_record_context(protocol_ranges);
	let ev_cb_data = EvCallbackData {
		xdpy: display,
		recdpy: recordeddpy,
		ctx,
		stop_key,
		nr_evs: 0,
		working: true
	};

	if !recordeddpy.enable_context_async(ctx, Some(ev_callback), addr_of!(ev_cb_data) as *mut c_char) {
		panic!("Failed to enable record context")
	}
	while ev_cb_data.working {
		recordeddpy.process_replies();
	}

	display.disable_context(ctx);
	display.free_context(ctx);
	unsafe { XFree(protocol_ranges as *mut c_void) };
}

#[derive(Debug)]
#[repr(C)]
pub struct EvCallbackData {
	pub xdpy: XDisplay,
	pub recdpy: XDisplay,
	pub stop_key: Keycode,
	pub nr_evs: u32,
	pub working: bool,
	pub ctx: XRecordContext,
	// x: i32,
	// y: i32,
}


unsafe extern "C" fn ev_callback(closure: *mut c_char, intercept_data: *mut XRecordInterceptData) {
	dbg!(intercept_data);
	dbg!(closure);
	let data = &mut *(closure as *mut EvCallbackData);
	let intercept_data = &mut *intercept_data;
	dbg!(&intercept_data);
	dbg!(&data);

	if intercept_data.category == XRecordStartOfData {
		println!("Got start of data!");
		return;
	} else if intercept_data.category == XRecordEndOfData {
		println!("Got end of data!");
		return;
	}
	data.nr_evs += 1;
	// println!("nr: {:?}, len: {:?}", data, intercept_data.data_len);

	dbg!(intercept_data.data);
	let ev_type = *(intercept_data.data as *const u8);
	match ev_type {
		KEYPRESS_U8 => {
			let kc = *((intercept_data.data as usize + 1) as *const u8);
			let stop = kc == data.stop_key as u8;
			if stop {
				println!("stop key detected!");
				data.working = false;
				XRecordFreeData(intercept_data)
			} else {
				// let keyname = data.xdpy.keycode_to_string(kc as u32);
				let keyname = data.xdpy.keycode_to_string(44);
				let rstring = format!("KeyPress {}", &keyname.to_str().unwrap());
				// let rstring = format!("KeyPress {}", kc);
				// dbg!(kc);
				dbg!(&rstring);
				std::mem::forget(keyname);
				// XRecordFreeData(intercept_data)
			}
		}
		KEYRELEASE_U8 => {}
		BUTTONPRESS_U8 => {}
		BUTTONRELEASE_U8 => {}
		MOTIONNOTIFY_U8 => {}
		_ => eprintln!("Unknown event type: {:?}", ev_type)
	}

}

const KEYPRESS_U8: u8 = KeyPress as u8;
const KEYRELEASE_U8: u8 = KeyRelease as u8;
const BUTTONPRESS_U8: u8 = ButtonPress as u8;
const BUTTONRELEASE_U8: u8 = ButtonRelease as u8;
const MOTIONNOTIFY_U8: u8 = MotionNotify as u8;

