#![no_std]

#![feature(asm)]
#![feature(global_asm)]
#![feature(naked_functions)]

use core::mem::transmute;

use riscv::register::mtvec::{TrapMode, self};

mod default_interrupts;
mod vector_table;

static mut VAR: u8 = 0;

#[no_mangle]
#[link_section = ".text.entry"]
#[naked]
pub unsafe extern "C" fn __entry() -> ! {
	asm!(r#"
	////////////////////////////////////////////////////////////////////////////
	// Load global pointer and stack pointer
	la    gp, __global_pointer
	la    sp, __stack_top

	la    a0, {VAR}

	////////////////////////////////////////////////////////////////////////////
	// Load .data RAM from data flash ROM

	la    t0, __data_start
	la    t1, __data_lma
	la    t2, __data_size
	add   t2, t2, t0

1:
	lw    t4, 0(t1)
	sw    t4, 0(t0)
	addi  t1, a1, 4
	addi  t0, t0, 4
	bltu  t0, t2, 1b

	////////////////////////////////////////////////////////////////////////////
	// Wipe .bss

	la    t0, __edata
	la    t1, __end
1:
	sw    x0, 0(t0)
	addi  t0, t0, 4
	bltu  t0, t1, 1b

	////////////////////////////////////////////////////////////////////////////
	// Jump to Rust

	j     {__rust_entry}
	"#,
		__rust_entry = sym __rust_entry,
		VAR = sym VAR,
		options(noreturn)
	);
}

#[no_mangle]
pub unsafe extern "C" fn __rust_entry() -> ! {
	extern "Rust" {
		fn main() -> !;
	}

	mtvec::write(transmute(vector_table::vector_table as unsafe extern "C" fn() -> _), TrapMode::Vectored);

	main()
}
