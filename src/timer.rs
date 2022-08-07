use core::ptr::{read_volatile, write_volatile};

pub trait Oneshot {
	fn start(&self, us: u64);
}

pub struct MTimer {
}

impl MTimer {
	pub fn new() -> Self {
		MTimer {}
	}

	pub fn get_current(&self) -> usize {
		unsafe { read_volatile(0x200BFF8 as *const usize) }
	}

	pub fn interrupt_handler(&self) {
	}
}

impl Oneshot for MTimer {
	fn start(&self, us: u64) {
		unsafe {
			let current_time = read_volatile(0x200BFF8 as *const u64);
			write_volatile(0x2004000 as *mut u64, (current_time + us) as u64);
		};
	}
}