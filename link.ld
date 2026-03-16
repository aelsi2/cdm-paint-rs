OUTPUT_FORMAT("binary")

SECTIONS
{
  .ivt 0x00 : {     /* Interrupt vector table */
    KEEP(*(.ivt))
  }

  .mmio 0x80 : {    /* Memory-mapped registers */
    KEEP(*(.mmio))
  }

  .text : {
    *(.text*)       /* Program code */
    *(.rodata*)     /* Read-only data */
  }

  .data : {
    *(.data*)       /* Initialized variables */
  }

  .bss : {
    *(.bss*)        /* Uninitialized variables */
    *(COMMON)
  }
}
