.global dispatch_task
.global get_current_msp
.cpu cortex-m4
.syntax unified
.thumb

dispatch_task:
	mrs ip, psr
  	push {R4-R11, ip,lr}
	msr psp, r0
	mov r0, #3
	msr control, r0
	pop {R4-R11, ip,lr}
	bx lr

get_current_msp:
	bkpt
	mrs r0, msp
	bx lr