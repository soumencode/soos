ENTRY(_start)

SECTIONS
{
	.text :
	{
        . = ALIGN(4);
		_stext = .;
		*(.text .text.*);
        . = ALIGN(4);
		_etext = .;

        . = ALIGN(4);
		_srodata = .;
		*(.rodata .rodata.*);
        . = ALIGN(4);
		_erodata = .;
	} > prog

	.data :
	{
        . = ALIGN(4);
		_sdata = .;
		*(.data .data.*);
        . = ALIGN(4);
		_edata = .;

        . = ALIGN(4);
		_sbss = .;
        *(.bss .bss.*);
        . = ALIGN(4);
		_ebss = .;

		. = ALIGN(4);
		_sstack = .;
		KEEP(*(.stack))
		. = ALIGN(4);
		_estack = .;
	} > data
}
