.cpu cortex-m4
.syntax unified
.thumb

.global syscall
syscall:
    bkpt
    svc 0 ;

.global __invoke
__invoke:
    bkpt
    mov r9, r0
    svc 1 ;    
    bx lr

.global __start_kernel
__start_kernel:
    ldmia r0, {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    mrs r9, psp
    msr psp, r0
	msr control, r4
    isb
    bkpt
    bx lr

.type SVCall, %function
.global SVCall
SVCall:
    bkpt
    /* save calling process */
    mrs r0, psp
    mov r1, #0xfffffffd
    cmp r1, lr
    IT EQ
    stmiaeq r0, {r4, r5, r6, r7, r8, r9, r10, r11, lr}

    /* load target process */
    ldmia r9, {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    @ mrs r9, psp
    @ sub r9, #8
    msr psp, r9
    isb

	msr control, r4
    isb

    mov lr, #0xfffffffd
    @ push {r0}
    @ pop {pc}
    bx lr
    
    
    
    
    
    
    
    
    
    
    
    
    @ push {lr}
    @ mov r7, sp
    @ bl __schedule
    @ b.n 0x8001236
    @ add pc, #0x2
    @ pop {r7}
    @ bx lr

@     bkpt
@     bx lr
@     /* save user state */
@     @ stmia r9, {r4, r5, r6, r7, r8, r9, r10, r11, lr} 

@     /* load kernel state */ 
@     @ pop {r4, r5, r6, r7, r8, r9, r10, r11, ip, lr}
@     @ msr xpsr, ip
@     @ isb

@     @ mrs r0, msp
@     @ mov r1, #0xC
@     @ sub r0, r0, r1
@     @ msr msp, r0

@     @ mov r0, #0x0
@ 	@ msr control, r0
@     @ isb 
@     @ mov lr, #0xFFFFFFF9

@     @ mov pc, lr


.global __schedule
__schedule:
    bkpt