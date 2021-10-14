

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
EXTERN(EXCEPTIONS);
SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    LONG(ORIGIN(SRAM) + LENGTH(SRAM));
    KEEP(*(.vector_table.reset_vector));
    KEEP(*(.vector_table.exceptions));
  } > FLASH
  PROVIDE(NMI = DefaultExceptionHandler);
  PROVIDE(HardFault = DefaultExceptionHandler);
  PROVIDE(MemManage = DefaultExceptionHandler);
  PROVIDE(BusFault = DefaultExceptionHandler);
  PROVIDE(UsageFault = DefaultExceptionHandler);
  PROVIDE(SVCall = DefaultExceptionHandler);
  PROVIDE(PendSV = DefaultExceptionHandler);
  .text :
  {
    *(.text .text.*);
  } > FLASH

  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  .bss :
  {
    _sbss = .;
    *(.bss .bss.*);
    _ebss = .;
  } > SRAM

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