MEMORY
{
	prog(rwx) : ORIGIN = 0x80000000, LENGTH = 0x20000000
	data(rwx) : ORIGIN = 0xA0000000, LENGTH = 0x20000000
}

ENTRY(_start)

SECTIONS
{
	.start :
	{
		KEEP(*(.start));
	} > prog

	.text :
	{
        . = ALIGN(8);
		_stext = .;
		*(.text .text.*);
        . = ALIGN(8);
		_etext = .;

        . = ALIGN(8);
		_srodata = .;
		*(.rodata .rodata.*);
        . = ALIGN(8);
		_erodata = .;
	} > prog

	.data :
	{
        . = ALIGN(8);
		_sdata = .;
		*(.data .data.*);
        . = ALIGN(8);
		_edata = .;

        . = ALIGN(8);
		_sbss = .;
        *(.bss .bss.*);
        . = ALIGN(8);
		_ebss = .;
	} > data

	.stack (NOLOAD) :
	{
        . = ALIGN(8);
		_sstack = .;
		KEEP(*(.stack_memory))
        . = ALIGN(8);
		_estack = .;
	} > data
}
