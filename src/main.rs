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
		asm!(
			"csrrw sp, mscratch, sp",
			"bnez sp, 300f",

			// kernel
			"100:",
			"csrrw t0, mscratch, zero",

			"addi sp, sp, -16*4",
			"sw x1,   0*4(sp)", // ra
			"sw x5,   1*4(sp)", // t0
			"sw x6,   2*4(sp)", // t1
			"sw x7,   3*4(sp)", // t2
			"sw x28,  4*4(sp)", // t3
			"sw x29,  5*4(sp)", // t4
			"sw x30,  6*4(sp)", // t5
			"sw x31,  7*4(sp)", // t6
			"sw x10,  8*4(sp)", // a0
			"sw x11,  9*4(sp)", // a1
			"sw x12, 10*4(sp)", // a2
			"sw x13, 11*4(sp)", // a3
			"sw x14, 12*4(sp)", // a4
			"sw x15, 13*4(sp)", // a5
			"sw x16, 14*4(sp)", // a6
			"sw x17, 15*4(sp)", // a7

			// jal ra, _start_trap_rust_from_kernel

			"lw x1,   0*4(sp)", // ra
			"lw x5,   1*4(sp)", // t0
			"lw x6,   2*4(sp)", // t1
			"lw x7,   3*4(sp)", // t2
			"lw x28,  4*4(sp)", // t3
			"lw x29,  5*4(sp)", // t4
			"lw x30,  6*4(sp)", // t5
			"lw x31,  7*4(sp)", // t6
			"lw x10,  8*4(sp)", // a0
			"lw x11,  9*4(sp)", // a1
			"lw x12, 10*4(sp)", // a2
			"lw x13, 11*4(sp)", // a3
			"lw x14, 12*4(sp)", // a4
			"lw x15, 13*4(sp)", // a5
			"lw x16, 14*4(sp)", // a6
			"lw x17, 15*4(sp)", // a7
			"addi sp, sp, 16*4",

			"mret",

			// app
			"300:",
			"sw s0, 0*4(sp)",
			"lw s0, 1*4(s0)",
			"sw x1,   0*4(s0)",
			"sw x3,   2*4(s0)",
			"sw x4,   3*4(s0)",
			"sw x5,   4*4(s0)",
			"sw x6,   5*4(s0)",
			"sw x7,   6*4(s0)",
			"sw x9,   8*4(s0)",
			"sw x10,  9*4(s0)",
			"sw x11, 10*4(s0)",
			"sw x12, 11*4(s0)",
			"sw x13, 12*4(s0)",
			"sw x14, 13*4(s0)",
			"sw x15, 14*4(s0)",
			"sw x16, 15*4(s0)",
			"sw x17, 16*4(s0)",
			"sw x18, 17*4(s0)",
			"sw x19, 18*4(s0)",
			"sw x20, 19*4(s0)",
			"sw x21, 20*4(s0)",
			"sw x22, 21*4(s0)",
			"sw x23, 22*4(s0)",
			"sw x24, 23*4(s0)",
			"sw x25, 24*4(s0)",
			"sw x26, 25*4(s0)",
			"sw x27, 26*4(s0)",
			"sw x28, 27*4(s0)",
			"sw x29, 28*4(s0)",
			"sw x30, 29*4(s0)",
			"sw x31, 30*4(s0)",
			"lw t0, 0*4(s0)",
			"sw t0, 7*4(s0)",

			"csrr t0, mscratch",
			"sw t0, 1*4(sp)",
			"csrr t0, mepc",
			"sw t0, 31*4(sp)",
			"csrr t0, mtval",
			"sw t0, 33*4(s0)",
			"csrr t0, mcause",
			"sw t0, 32*4(s0)",

			"bge t0, zero, 200f",
			"mv a0, t0",

			//"jal ra, _disable_interrupt_trap_rust_from_app",

			// app continue
			"200:",
			"lw t0, 2*4(sp)",
			"csrw mepc, t0",
			"csrw mscratch, zero",
			"csrr t0, mstatus",
			"li t1, 0x1800",
			"or t0, t0, t1",
			"csrw mstatus, t0",
			"mret",
			options(noreturn)
		)
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

struct Frame {
	// x0:zero
	// x1: ra: return address
	// x2: sp: stack pointer
	// x3: gp: global pointer
	// x4: tp: thread pointer
	// x5-x7: t0-t2: temporaries
	// x8: s0/fp: saved register/frame pointer
	// x9: s1: saved register
	// x10-x11: a0-a1: function arguments/return values
	// x12-x17: a2-a7: function arguments
	// x18-x27: s2-s11: saved registers
	// x28-x31: t3-t6: temporaries
	pub regs: [usize;32],
}

impl Frame {
	pub fn zero() -> Self {
		Frame {
			regs: [0;32],
		}
	}
}

struct Process {
	frame: Frame,
	stack: *mut usize,
}

impl Process {
	pub fn new(entry: fn()) -> Self {
		Process {
			stack: 0 as *mut usize,
			frame: Frame::zero(),
		}
	}
}

unsafe fn switch_process() {
	asm!(
		// spにはkernelのspが入っている
		"addi sp, sp, -34*4",
		"sw x1,  3*4(sp)",
		// x2
		"sw x3,  4*4(sp)",
		"sw x4,  5*4(sp)",
		"sw x5,  6*4(sp)",
		"sw x6,  7*4(sp)",
		"sw x7,  8*4(sp)",
		"sw x8,  9*4(sp)",
		"sw x9,  10*4(sp)",
		"sw x10, 11*4(sp)",
		"sw x11, 12*4(sp)",
		"sw x12, 13*4(sp)",
		"sw x13, 14*4(sp)",
		"sw x14, 15*4(sp)",
		"sw x15, 16*4(sp)",
		"sw x16, 17*4(sp)",
		"sw x17, 18*4(sp)",
		"sw x18, 19*4(sp)",
		"sw x19, 20*4(sp)",
		"sw x20, 21*4(sp)",
		"sw x21, 22*4(sp)",
		"sw x22, 23*4(sp)",
		"sw x23, 24*4(sp)",
		"sw x24, 25*4(sp)",
		"sw x25, 26*4(sp)",
		"sw x26, 27*4(sp)",
		"sw x27, 28*4(sp)",
		"sw x28, 29*4(sp)",
		"sw x29, 30*4(sp)",
		"sw x30, 31*4(sp)",
		"sw x31, 32*4(sp)",
		// なにのために必要なのかイマイチ不明
		"sw a0, 1*4(s0)",

		// MPP=USER, MIE=enable
		"li t0, 0x00001808",
		"csrrc x0, mstatus, t0",
		// MPIE=enable
		"li t0, 0x00000080",
		"csrrs x0, mstatus, t0",
		//
		"lui t0, %hi(100f)",
		"addi t0, t0, %lo(100f)",
		// 次の実行pcを保存
		"sw t0, 2*4(sp)",
		// kernelのspを保存する
		"csrw mscratch, sp",

		"lw t0, 31*4(a0)",
		"csrw mepc, t0",

		// appのレジスタに切り替える
		"mv t0,   a0", // t0==x5
		"lw x1,   0*4(t0)",
		"lw x2,   1*4(t0)",
		"lw x3,   2*4(t0)",
		"lw x4,   3*4(t0)",
		"lw x6,   5*4(t0)",
		"lw x7,   6*4(t0)",
		"lw x8,   7*4(t0)",
		"lw x9,   8*4(t0)",
		"lw x10,  9*4(t0)",
		"lw x11, 10*4(t0)",
		"lw x12, 11*4(t0)",
		"lw x13, 12*4(t0)",
		"lw x14, 13*4(t0)",
		"lw x15, 14*4(t0)",
		"lw x16, 15*4(t0)",
		"lw x17, 16*4(t0)",
		"lw x18, 17*4(t0)",
		"lw x19, 18*4(t0)",
		"lw x20, 19*4(t0)",
		"lw x21, 20*4(t0)",
		"lw x22, 21*4(t0)",
		"lw x23, 22*4(t0)",
		"lw x24, 23*4(t0)",
		"lw x25, 24*4(t0)",
		"lw x26, 25*4(t0)",
		"lw x27, 26*4(t0)",
		"lw x28, 27*4(t0)",
		"lw x29, 28*4(t0)",
		"lw x30, 29*4(t0)",
		"lw x31, 30*4(t0)",
		"lw  x5,  4*4(t0)",

		// mepcが指すappに飛ぶ
		"mret",

		// ここはkernel
		// kernelのレジスタに切り替える
		// appレジスタはtrap内で保存済
		"100:",
		"lw x1,   3*4(sp)",
		"lw x3,   4*4(sp)",
		"lw x4,   5*4(sp)",
		"lw x5,   6*4(sp)",
		"lw x6,   7*4(sp)",
		"lw x7,   8*4(sp)",
		"lw x8,   9*4(sp)",
		"lw x9,  10*4(sp)",
		"lw x10, 11*4(sp)",
		"lw x11, 12*4(sp)",
		"lw x12, 13*4(sp)",
		"lw x13, 14*4(sp)",
		"lw x14, 15*4(sp)",
		"lw x15, 16*4(sp)",
		"lw x16, 17*4(sp)",
		"lw x17, 18*4(sp)",
		"lw x18, 19*4(sp)",
		"lw x19, 20*4(sp)",
		"lw x20, 21*4(sp)",
		"lw x21, 22*4(sp)",
		"lw x22, 23*4(sp)",
		"lw x23, 24*4(sp)",
		"lw x24, 25*4(sp)",
		"lw x25, 26*4(sp)",
		"lw x26, 27*4(sp)",
		"lw x27, 28*4(sp)",
		"lw x28, 29*4(sp)",
		"lw x29, 30*4(sp)",
		"lw x30, 31*4(sp)",
		"lw x31, 32*4(sp)",

		"addi sp, sp, 34*4",
	)
	
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
		//let current = timer.get_current();
		//write!(uart, "{}\n", current);
	}
}
