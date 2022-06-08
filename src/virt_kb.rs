use std::sync::mpsc;
use std::sync::mpsc::{Sender, SendError};
use std::thread;
use evdev::{Device, InputEvent};
use evdev::uinput::VirtualDeviceBuilder;

pub struct VirtKb {
	tx: Sender<Vec<InputEvent>>,
}

impl VirtKb {
	pub fn new(physical_kbs: &Vec<Device>) -> Self {
		let (tx, rx) = mpsc::channel::<Vec<InputEvent>>();

		let mut virt_kb_builder = VirtualDeviceBuilder::new()
			.expect("Failed to create UInput device")
			.name("EasyMacros");

		for phys_kb in physical_kbs {
			virt_kb_builder = virt_kb_builder.with_keys(phys_kb.supported_keys().unwrap()).unwrap();
		}

		let mut virt_kb = virt_kb_builder.build().expect("Failed to build virt_kb");

		thread::spawn(move || {
			for received in rx {
				virt_kb.emit(&received).expect("Virtual keyboard failed to send events");
			}
		});

		Self { tx }
	}

	pub fn send_events(&self, events: Vec<InputEvent>) -> Result<(), SendError<Vec<InputEvent>>> {
		self.tx.send(events)
	}
}