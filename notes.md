- MSP main stack pointer
- PSR program status register (APSR, EPSR and IPSR)
- MRS move the contents of a special register to a general purpose register
- Reset_Handler() Initialize PC (program counter)
- ARM vs Thumb state [Link](https://developer.arm.com/documentation/dui0040/d/CACCIDAH)
    - destination leading bit indicated Thumb state
- stack frame
    - When the processor takes an exception, unless the exception is a tail-chained or a late-
arriving exception, the processor pushes information onto the current stack. This operation 
is referred as stacking and the structure of eight data words is referred as stack frame. (p42)


Stack anschauen
x/10x <MSP>
x/10x 0x20003f58

layout asm
layout split
layout src
layout regs

