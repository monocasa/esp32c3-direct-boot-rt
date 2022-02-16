global_asm!(r#"
.section .text

.weak interrupt_1_entry
interrupt_1_entry:
.weak interrupt_2_entry
interrupt_2_entry:
.weak interrupt_3_entry
interrupt_3_entry:
.weak interrupt_4_entry
interrupt_4_entry:
.weak interrupt_5_entry
interrupt_5_entry:
.weak interrupt_6_entry
interrupt_6_entry:
.weak interrupt_7_entry
interrupt_7_entry:
.weak interrupt_8_entry
interrupt_8_entry:
.weak interrupt_9_entry
interrupt_9_entry:
.weak interrupt_10_entry
interrupt_10_entry:
.weak interrupt_11_entry
interrupt_11_entry:
.weak interrupt_12_entry
interrupt_12_entry:
.weak interrupt_13_entry
interrupt_13_entry:
.weak interrupt_14_entry
interrupt_14_entry:
.weak interrupt_15_entry
interrupt_15_entry:
.weak interrupt_16_entry
interrupt_16_entry:
.weak interrupt_17_entry
interrupt_17_entry:
.weak interrupt_18_entry
interrupt_18_entry:
.weak interrupt_19_entry
interrupt_19_entry:
.weak interrupt_20_entry
interrupt_20_entry:
.weak interrupt_21_entry
interrupt_21_entry:
.weak interrupt_22_entry
interrupt_22_entry:
.weak interrupt_23_entry
interrupt_23_entry:
.weak interrupt_24_entry
interrupt_24_entry:
.weak interrupt_25_entry
interrupt_25_entry:
.weak interrupt_26_entry
interrupt_26_entry:
.weak interrupt_27_entry
interrupt_27_entry:
.weak interrupt_28_entry
interrupt_28_entry:
.weak interrupt_29_entry
interrupt_29_entry:
.weak interrupt_30_entry
interrupt_30_entry:
.weak interrupt_31_entry
interrupt_31_entry:
	mret

.weak exception_entry
exception_entry:
	j     exception_entry

"#);
