use std::cell::{Ref, RefCell};
use std::env;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong};
use x11::xlib::{Display, GenericEvent, KeyCode, KeyPress, MotionNotify, Status, Time, Window, XAllowEvents, XAnyEvent, XCloseDisplay, XDefaultScreen, XEvent, XFlush, XGrabKeyboard, XKeycodeToKeysym, XKeyEvent, XKeysymToKeycode, XKeysymToString, XOpenDisplay, XRootWindow, XStringToKeysym, XSync, XUngrabKeyboard, XUngrabPointer, XWindowEvent};
use x11::xrecord::{XRecordAllClients, XRecordAllocRange, XRecordClientInfo, XRecordClientSpec, XRecordContext, XRecordCreateContext, XRecordDisableContext, XRecordEnableContext, XRecordEnableContextAsync, XRecordFreeContext, XRecordInterceptData, XRecordProcessReplies, XRecordQueryVersion, XRecordRange};
use x11::xtest::{
	XTestFakeButtonEvent, XTestFakeKeyEvent, XTestFakeMotionEvent, XTestGrabControl,
	XTestQueryExtension,
};

#[derive(Debug, Clone, Copy)]
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
		}).unwrap();
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

	pub fn get_default_screen(&self) -> c_int {
		unsafe { XDefaultScreen(self.ptr) }
	}

	pub fn get_root_window(&self, screen_nr: c_int) -> Window {
		unsafe { XRootWindow(self.ptr, screen_nr) }
	}

	pub fn keysym_to_keycode(&self, keysym: Keysym) -> Keycode {
		unsafe { XKeysymToKeycode(self.ptr, keysym) as Keycode }
	}

	pub fn string_to_keycode(&self, string: &[u8]) -> Keycode {
		self.keysym_to_keycode(string_to_keysym(string))
	}

	pub fn keycode_to_keysym(&self, keycode: Keycode) -> Keysym {
		unsafe {
			XKeycodeToKeysym(self.ptr, keycode as c_uchar, 0)
		}
	}

	pub fn keycode_to_string(&self, keycode: Keycode) -> CString {
		keysym_to_string(self.keycode_to_keysym(keycode))
	}

	// XTest stuff
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

	pub fn send_fake_motion_event(&self, x: i32, y: i32) {
		unsafe { XTestFakeMotionEvent(self.ptr, -1, x, y, 10) };
		self.flush();
	}

	pub fn grab_control(&self) {
		unsafe {
			XTestGrabControl(self.ptr, TRUE_C);
		}
	}

	pub fn allow_events(&self, event_mode: i32, time: Time) {
		unsafe { XAllowEvents(self.ptr, event_mode, time) };
	}

	pub fn grab_keyboard(&self, window: u64, owner_events: bool, pointer_mode: i32, keyboard_mode: i32, time: Time) -> i32 {
		unsafe {
			XGrabKeyboard(
                self.ptr,
                window,
                c_int::from(owner_events),
                pointer_mode,
                keyboard_mode,
                time,
            )
		}
	}

	pub fn ungrab_keyboard(&self, time: Time) {
		unsafe { XUngrabKeyboard(self.ptr, time) };
	}

	pub fn ungrab_pointer(&self, time: Time) {
		unsafe { XUngrabPointer(self.ptr, time) };
	}
	pub fn window_event(&self, window: Window, event_mask: i64) -> XEvent {
		// maybe dirty hack to initialize the event var?? idk how else to do this
		let mut r: XEvent = XEvent { type_: GenericEvent };

		unsafe { XWindowEvent(self.ptr, window, event_mask, &mut r); }

		r
	}

	// XRecord stuff
	pub fn has_xrecord(&self) -> bool {
		let mut xrecord_version: (c_int, c_int) = (0, 0);
		let xrec_res = unsafe { XRecordQueryVersion(self.ptr, &mut xrecord_version.0, &mut xrecord_version.1) };
		xrec_res == 0
	}

	pub fn create_record_context(&self, mut protocol_ranges: *mut XRecordRange) -> XRecordContext {
		unsafe {
			(*protocol_ranges).device_events.first = KeyPress as c_uchar;
			(*protocol_ranges).device_events.last = MotionNotify as c_uchar;
		}
        let mut clients: XRecordClientSpec = XRecordAllClients;

		let ctx: XRecordContext = unsafe {
            XRecordCreateContext(
                self.ptr,
                0,
                &mut clients,
                1,
                &mut protocol_ranges,
                1
            )
        };
		ctx
	}

	pub fn enable_context(&self,
						  ctx: XRecordContext,
						  cb:Option<unsafe extern "C" fn(_: *mut c_char, _: *mut XRecordInterceptData)>,
						  closure: *mut c_char
	) -> bool {
		unsafe {
			XRecordEnableContext( self.ptr, ctx, cb, closure as *mut c_char ) != 0
		}
	}

    pub fn enable_context_async(&self,
								ctx: XRecordContext,
								cb: Option<unsafe extern "C" fn(_: *mut c_char, _: *mut XRecordInterceptData)>,
								closure: *mut c_char,
    ) -> bool {
        unsafe {
            XRecordEnableContextAsync( self.ptr, ctx, cb, closure as *mut c_char ) != 0
        }
    }

	pub fn disable_context(&self, ctx: XRecordContext) -> bool {
		unsafe {
			XRecordDisableContext( self.ptr, ctx ) != 0
		}
	}

	pub fn free_context(&self, ctx: XRecordContext) -> bool {
		unsafe {
			XRecordFreeContext( self.ptr, ctx) != 0
		}
	}

    pub fn process_replies(&self) {
        unsafe { XRecordProcessReplies(self.ptr) };
    }
}

/// Wrapper for XStringToKeysym. Remember to give a null terminated string!
pub fn string_to_keysym(string: &[u8]) -> Keysym {
	unsafe { XStringToKeysym(string.as_ptr() as *const c_char) }
}
pub fn keysym_to_string(keysym: Keysym) -> CString {
	unsafe {
		let raw = XKeysymToString(keysym);
		dbg!(raw);
		let r = CString::from_raw(raw);
		r
	}
		// .into_string()
		// .expect("failed to convert CString into Rust String")
}
