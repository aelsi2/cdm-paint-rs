OUTPUT_FORMAT("binary")

SECTIONS
{
  .ivt : {
    KEEP(*(.ivt))
  }
  . = 0x80;
  .mmio : {
    KEEP(*(.mmio))
  }

  .text : {
    *(.text*)      /* Program code */
    *(.rodata*)    /* Read-only data */
  }

  .data : {
    *(.data*)      /* Initialized variables */
  }

  .bss : {
    *(.bss*)       /* Uninitialized variables */
    *(COMMON)
  }
}
