#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(naked_functions)]

use core::arch::asm;

#[naked]
#[export_name = "_start"]
extern "C" fn _start() {
    unsafe {
        asm!("j main",
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
    loop {}
}
