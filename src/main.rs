use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use evdev::{Device, enumerate, InputEvent};
use crate::modifiers::ModifierStates;
use crate::virt_kb::VirtKb;

mod modifiers;
mod virt_kb;

fn main() {
	let devices = enumerate();
	let kb_name = "py-evdev-uinput";

	let mut kbs = {
		let mut r = Vec::new();

		for dev in devices {
			if let Some(keyboard) = dev.name() {
				if keyboard == kb_name {
					r.push(dev);
				}
			}
		}

		r
	};


	let virt_kb = VirtKb::new(&kbs);
	let mod_states = ModifierStates::new();

	let reader = KeyReader::new(kbs);
	let event_receiver = reader.init();


}

struct KeyReader {
	keyboards: Arc<Mutex<Vec<Device>>>,
}

impl KeyReader {
	pub fn new(kbs: Vec<Device>) -> Self {
		let mut kbs = kbs;

		for kb in kbs.iter_mut() {
			match kb.grab() {
				Ok(_) => println!("Keyboard {} grabbed!", kb.name().get_or_insert("[Unnamed]")),
				Err(_) => println!("Failed to grab keyboard {}.", kb.name().get_or_insert("[Unnamed]")),
			}
		}

		Self { keyboards: Arc::new(Mutex::new(kbs)) }
	}

	pub fn init(&self) -> Receiver<Vec<InputEvent>>{
		let (tx, rx) = mpsc::channel();

		let keyboards = self.keyboards.clone();
		thread::spawn(move || {
			let mut kbs_locked = keyboards.lock().unwrap();
			loop {
				for kb in kbs_locked.iter_mut() {
					let mut events = kb
						.fetch_events()
						.unwrap()
						.collect::<Vec<InputEvent>>();

					tx.send(events)
						.expect("Couldn't send keyboard events from reader to main thread.");
				}
			}
		});

		rx
	}
}