MEMORY
{
	/* top half of internal SRAM 1 (inst bus) */
	prog(rx) : ORIGIN = 0x40380000, LENGTH = 0x30000
	/* bottom half of internal SRAM 1 (data bus) */
	data(rwx) : ORIGIN = 0x3FCB0000, LENGTH = 0x30000
}

INCLUDE src/kernel.ld
