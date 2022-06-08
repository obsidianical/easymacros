use evdev::{InputEvent, Key};

#[derive(Debug)]
pub struct ModifierStates {
	ctrl: bool,
	alt: bool,
	shift: bool,
	meta: bool,
}

impl ModifierStates {
	pub fn new() -> Self {
		ModifierStates {
			ctrl: false,
			alt: false,
			shift: false,
			meta: false
		}
	}
	pub fn update(&mut self, event: InputEvent) {
		let code = event.code();
		let val = event.value();

		self.update_ctrl(code, val);
		self.update_alt(code, val);
		self.update_shift(code, val);
		self.update_meta(code, val);
	}
	pub fn mod_pressed(&self) -> bool {
		self.ctrl || self.alt || self.shift || self.meta
	}

	fn update_ctrl(&mut self, code: u16, val: i32) {
		if code == Key::KEY_LEFTCTRL.code() || code == Key::KEY_RIGHTCTRL.code() {
			self.ctrl = val != 0;
		}
	}

	fn update_alt(&mut self, code: u16, val: i32) {
		if code == Key::KEY_LEFTALT.code() || code == Key::KEY_RIGHTALT.code() {
			self.ctrl = val != 0;
		}
	}

	fn update_shift(&mut self, code: u16, val: i32) {
		if code == Key::KEY_LEFTSHIFT.code() || code == Key::KEY_RIGHTSHIFT.code() {
			self.ctrl = val != 0;
		}
	}

	fn update_meta(&mut self, code: u16, val: i32) {
		if code == Key::KEY_LEFTMETA.code() || code == Key::KEY_RIGHTMETA.code() {
			self.ctrl = val != 0;
		}
	}
}

