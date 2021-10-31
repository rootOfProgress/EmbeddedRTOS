.cpu cortex-m4
.syntax unified
.thumb

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
