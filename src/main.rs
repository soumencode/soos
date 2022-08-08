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
			// enable machine interrupt
			"csrr t0, mstatus",
			"ori t0, t0, 0x8",
			"csrw mstatus, t0",
			// enable timer interrupt
			"csrr t0, mie",
			"ori t0, t0, 0x1<<7",
			"csrw mie, t0",
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

//    unsafe {
        // asm!("csrrw sp, mscratch, sp", // mscratchとspを入れ替える
		// 	 "bneq sp, 300f", // 
		// 	 "csrrw sp, mscratch sp", // mscratchとspを入れ替える

		// 	 // kernel
		// 	 "csrrw t0, mscratch, zero",
		// 	 "addi sp, sp, -16*4",
		// 	 "sw ra, 0*4(sp)",
		// 	 "...",
		// 	 "jal ra, _start_trap_rust_from_kernel",
		// 	 "lw ra, 0*4(sp)"
		// 	 "...",
		// 	 "addi sp, sp, 16*4",
		// 	 "mret",

		// 	 // app
		// 	 "sw s0, 0*4(sp)",
		// 	 "lw s0, 1*4(sp)",
		// 	 "sw x1, 0*4(s0)",
		// 	 "...",
		// 	 "lw t0, 0*4(sp)",
		// 	 "sw t0, 7*4(s0)",

		// 	 "csrr t0, mscratch",
		// 	 "sw t0, 1*4(s0)",
		// 	 "csrr t0, mepc",
		// 	 "sw t0, 31*4(s0)",
		// 	 "csrr t0, mtval",
		// 	 "sw t0, 33*4(s0)",
		// 	 "csrr t0, mcause",
		// 	 "sw t0, 32*4(s0)",

		// 	 "bge t0, zero, 200f",

		// 	 // app(exception)
		// 	 "mv a0, t0",
		// 	 "jal ra, _disable_interrupt_trap_rust_from_app",

		// 	 // app(interrupt)
		// 	 "lw t0, 2*4(sp)",
		// 	 "csrw mepc, t0",
		// 	 "csrw mscratch, zero",
		// 	 "csrr t0, mstatus",
		// 	 "li t1, 0x1800",
		// 	 "or t0, t0, t1",
		// 	 "csrw mstatus, t0",
		// 	 "mret",

		// 	 options(noreturn));
//    }
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
	timer.start(1000000);

    loop {
		let current = timer.get_current();
		write!(uart, "{}\n", current);
	}
}
