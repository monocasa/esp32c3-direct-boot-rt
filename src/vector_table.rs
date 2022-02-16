#[link_section = ".vector_table"]
#[naked]
pub(crate) unsafe extern "C" fn vector_table() -> ! {
	asm!("
		j     exception_entry
		j     interrupt_1_entry
		j     interrupt_2_entry
		j     interrupt_3_entry
		j     interrupt_4_entry
		j     interrupt_5_entry
		j     interrupt_6_entry
		j     interrupt_7_entry
		j     interrupt_8_entry
		j     interrupt_9_entry
		j     interrupt_10_entry
		j     interrupt_11_entry
		j     interrupt_12_entry
		j     interrupt_13_entry
		j     interrupt_14_entry
		j     interrupt_15_entry
		j     interrupt_16_entry
		j     interrupt_17_entry
		j     interrupt_18_entry
		j     interrupt_19_entry
		j     interrupt_20_entry
		j     interrupt_21_entry
		j     interrupt_22_entry
		j     interrupt_23_entry
		j     interrupt_24_entry
		j     interrupt_25_entry
		j     interrupt_26_entry
		j     interrupt_27_entry
		j     interrupt_28_entry
		j     interrupt_29_entry
		j     interrupt_30_entry
		j     interrupt_31_entry
	",
	options(noreturn))
}