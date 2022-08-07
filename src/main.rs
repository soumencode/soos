#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(asm_sym)]
#![feature(fn_align)]

use core::arch::asm;
mod uart;
mod timer;
use timer::Oneshot;
use riscv::register;
use core::fmt::Write;

extern "C" {
    static _stack: usize;
}

#[naked]
#[link_section = ".start"]
#[export_name = "_start"]
extern "C" fn _start() {
    unsafe {
        asm!(
			"la sp, {stack}",
			// enable interrupt
			"csrr t0, mstatus",
			"ori t0, t0, 0x8",
			"csrw mstatus, t0",
			// goto main
			"j main",
			stack = sym _stack,
			options(noreturn));
    }
}

#[naked]
#[repr(align(4))]
extern "C" fn _trap() {
	unsafe {
		asm!("1: j 1b",
			 options(noreturn));
	}
}

/* https://www.reddit.com/r/rust/comments/estvau/til_why_the_eh_personality_language_item_is */
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/* https://docs.rust-embedded.org/book/start/panicking.html */
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() -> ! {
    unsafe {
        register::mtvec::write(_trap as usize, register::mtvec::TrapMode::Direct);
    }

    unsafe {
        register::pmpaddr0::write(0xFFFFFFFF);
        register::pmpcfg0::set_pmp(0, register::Range::TOR, register::Permission::RWX, false);
    }

    let mut uart = uart::Uart::new();
	let timer = timer::MTimer::new();
	let current = timer.get_current();
	write!(uart, "{}", current);
	timer.start(10000);
    loop {}
}
