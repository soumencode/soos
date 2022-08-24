#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(asm_sym)]
#![feature(fn_align)]

use core::arch::asm;
mod timer;
mod uart;
use core::fmt::Write;
use riscv::register;
use timer::Oneshot;

#[no_mangle]
#[link_section = ".stack_memory"]
pub static mut STACK_MEMORY: [u8; 0x4000] = [0; 0x4000];

extern "C" {
    static _estack: u8;
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
			stack = sym _estack,
			options(noreturn));
    }
}

#[naked]
#[repr(align(4))]
extern "C" fn _trap() {
    unsafe {
        asm!(
            "999: j 999b",
            "csrrw sp, mscratch, sp",
            "bnez sp, 300f",
            // kernel->kernelの割り込み
            "100:",
            "csrrw t0, mscratch, zero",
            // calling conventionに従っている
            // 普通の関数呼び出しみたいな感じだろうか
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
            // 割り込みハンドラ
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
            // trapの終わり
            "mret",
            // app->kernelの割り込み
            "300:",
            // kernelのspにひとまず保存s0(x8)
            "sw s0, 0*4(sp)",
            // s0にappの保存先がある？
            // 上の(a)を見ろ
            // コンパイラが勝手に上書きしないのかな
            "lw s0,   1*4(s0)",
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
            // きちんのappのs0も保存
            "lw t0, 0*4(s0)",
            "sw t0, 7*4(s0)",
            // spにmscratch/mepc/mtval/mcauseを保存
            "csrr t0, mscratch",
            "sw t0, 1*4(sp)",
            "csrr t0, mepc",
            "sw t0, 31*4(sp)",
            "csrr t0, mtval",
            "sw t0, 33*4(s0)",
            "csrr t0, mcause",
            "sw t0, 32*4(s0)",
            //
            "bge t0, zero, 200f",
            "mv a0, t0",
            // exceptionの場合
            // esp32-c3の場合は死ぬ
            //"jal ra, _disable_interrupt_trap_rust_from_app",

            // 割り込みの場合
            "200:",
            // switchでappに移っているのだからswitchから再開
            // t0にはそのpcが入っている
            "lw t0, 2*4(sp)",
            "csrw mepc, t0",
            "csrw mscratch, zero",
            "csrr t0, mstatus",
            // MPP=Machine
            "li t1, 0x1800",
            "or t0, t0, t1",
            "csrw mstatus, t0",
            // swtichの再開
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

#[repr(C)]
struct Process {
    regs: [usize; 32],
    pc: *mut usize,
}

impl Process {
    pub fn new(entry: unsafe fn() -> !, s: *mut usize) -> Self {
        let mut process = Process {
            regs: [0; 32],
            pc: entry as *mut usize,
        };
		process.regs[1] = s as usize;
		process
    }
}

// pub struct Riscv32iStoredState, regs[31], pc, mcause, mtval

unsafe fn switch_process(process: &Process) {
    asm!(
        // kernelでここに来る
        // userにswitchする前にkernelのregisterを保存する
        "addi sp, sp, -34*4",

        "sw x1,  3*4(sp)",
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
        // userプロセスの情報も保存する
        // kernelに戻ったあとにuserプロセスのレジスタを保存するため(a)
        "sw a0, 1*4(sp)",

        // MPP=USER, MIE=enable
        "li t0, 0x00001808",
        "csrrc x0, mstatus, t0",
        // MPIE=enable
        "li t0, 0x00000080",
        "csrrs x0, mstatus, t0",
        // 次の実行pcを保存
        "lui t0, %hi(100f)",
        "addi t0, t0, %lo(100f)",
        "sw t0, 2*4(sp)",
        // kernelのspを保存する
        "csrw mscratch, sp",

        // userのpcをロードする
        // 前に実行していたpcが保存されているのでそれを取得する
        //"lw t0, 31*4(a0)",
        //"csrw mepc, t0",
        "csrw mepc, a1",

        // appのレジスタに切り替える
        "mv x5,   a0", // t0==x5
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

        in("a0") &process.regs as &[usize; 32],
        in("a1") process.pc as *mut usize,
        options(noreturn),
    );
}

unsafe fn process1_func() -> ! {
    let mut uart = uart::Uart::new();
    loop {
        write!(uart, "hello1\n");
    }
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

    loop {
        unsafe {
            let mut uart = uart::Uart::new();
            let mut process1 = Process::new(
                process1_func,
                /*0x80085000*/ (&_estack as *const u8 as usize - 0x2000) as *mut usize,
            );
            switch_process(&process1);
        }
    }
}
