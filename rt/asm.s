.cpu cortex-m4
.syntax unified
.thumb

// ARM semihosting operations
.equ SYS_WRITEC, 0x03
.equ SYS_WRITE0, 0x04
.equ SYS_READC,  0x07

.global __syscall
__syscall:
    svc 0;
    bx lr

.global __sprint
__sprint:
    mov r1, r0
    mov r0, SYS_WRITE0
    bkpt 0xAB ;
    bx lr

.global __sprintc
__sprintc:
    mov r1, r0
    mov r0, SYS_WRITEC
    bkpt 0xAB ;
    bx lr

.global __sreadc
__sreadc:
    mov r1, #0x0
    mov r0, SYS_READC
    bkpt 0xAB ;
    mov r5, r0
    bx lr

.global __get_r5
__get_r5:
    mov r0, r5
    bx lr

.global __get_r0
__get_r0:
    bx lr

.global __invoke
__invoke:
    @ bkpt
    /* r9 will have a pointer to the current ProcessFrame */
    mov r9, r0
    push {lr}
    bl trigger_PendSV
    pop {pc}


.type PendSV, %function
.global PendSV
PendSV:
    /* save called process */
    /* skip saving when called from msp */
    mov r1, #0xfffffffd
    cmp r1, lr
    ITT EQ
    mrseq r0, psp
    stmdbeq r0, {r4, r5, r6, r7, r8, r10, r11, lr}

    /* load target process */
    ldmdb r9, {r4, r5, r6, r7, r8, r10, r11, lr}
    msr psp, r9
    isb
    mov r9, r0
    
	msr control, r4
    isb
    
    bx lr

.global __save_psp
__save_psp:
    mov r0, r9
    bx lr

.global __breakpoint
__breakpoint:
    isb
    bkpt
