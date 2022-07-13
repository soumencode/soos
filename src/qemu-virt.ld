MEMORY
{
	prog(rx) : ORIGIN = 0x80000000, LENGTH = 0x30000
	data(rwx) : ORIGIN = 0x80030000, LENGTH = 0x30000
}

INCLUDE src/kernel.ld
