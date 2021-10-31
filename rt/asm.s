.cpu cortex-m4
.syntax unified
.thumb

.global syscall
syscall:
    bkpt
    svc 0;
    bx lr

.type PendSV, %function
.global PendSV
PendSV:
    bkpt
    bx lr

@ .type SysTick, %function
@ .global SysTick
@ SysTick:
@     push {lr}
@     bkpt
@     bl trigger_PendSV
@     isb
@     bkpt
@     pop {pc}


.global __invoke
__invoke:
    bkpt
    /* r9 will have a pointer to the current ProcessFrame */
    mov r9, r0
    svc 1 ;    
    bx lr

.type SVCall, %function
.global SVCall
SVCall:
    bkpt
    /* save calling process, only save when thread */
    /* r9 stores the current psp */
    
    /* Dont save psp when there is no one ... called from msp */
    mov r1, #0xfffffffd
    cmp r1, lr
    ITT EQ
    mrseq r0, psp
    stmdbeq r0, {r4, r5, r6, r7, r8 ,r10, r11, lr}

    /* load target process */
    ldmdb r9, {r4, r5, r6, r7, r8    ,r10, r11, lr}
    @ mrs r9, psp
    @ sub r9, #8
    msr psp, r9
    isb
    mov r9, r0
    

	msr control, r4
    isb

    @ mov lr, #0xfffffffd
    bx lr


.global __save_psp
__save_psp:
    mov r0, r9
    bx lr

.global __schedule
__schedule:
    isb
    bkpt