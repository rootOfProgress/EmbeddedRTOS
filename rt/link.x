

/* generall memory info; manual cortex m4 page 30 */
MEMORY
{
  /* see page 53 at https://www.st.com/resource/en/datasheet/stm32f303vc.pdf */
  /* flash area starts @0800000 */
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 32K

  /* also sram area starts @20000000 */
  SRAM (rwx) : ORIGIN = 0x20000000, LENGTH = 16K
}

ENTRY(Reset);

EXTERN(RESET_VECTOR);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    LONG(ORIGIN(SRAM) + LENGTH(SRAM));
    KEEP(*(.vector_table.reset_vector));
  } > FLASH

  /* .text is where executable code goes. */
  .text :
  {
    *(.text .text.*);
  } > FLASH

  /* static variables or string literals need .bss .data and .rodata sections */

  /* .rodata is read only data; it is where global constants are placed. */
  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  /* .bss is where uninitialized global variables are placed. */
  .bss :
  {
    _sbss = .;
    *(.bss .bss.*);
    _ebss = .;
  } > SRAM

  /* .data is where global variables that are initialized at compile time are placed. */
  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
  {
    _sdata = .;
    *(.data .data.*);
    _edata = .;
  } > SRAM

  _sidata = LOADADDR(.data);

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}