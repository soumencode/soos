MEMORY
{
	prog(rwx) : ORIGIN = 0x80000000, LENGTH = 0x80000
	data(rwx) : ORIGIN = 0x80080000, LENGTH = 0x80000
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
	} > data

	PROVIDE(_stack = . + 0x2000);
}

