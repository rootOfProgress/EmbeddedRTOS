

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

  .text :
  {
    *(.text .text.*);
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}