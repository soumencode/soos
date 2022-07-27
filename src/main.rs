#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(asm_sym)]
#![feature(fn_align)]

use core::arch::asm;

mod uart;
use riscv::register;

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
			"j main",
			stack = sym _stack,
			options(noreturn));
    }
}

#[naked]
#[repr(align(4))]
extern "C" fn _trap() {
    unsafe {
        asm!("1: j 1b", options(noreturn));
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

    let uart = uart::Uart::new();
    uart.send('G' as u8);
    loop {}
}
