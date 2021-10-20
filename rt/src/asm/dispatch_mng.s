.global dispatch_task
.global get_current_msp
.global get_current_psp
.global _save_process_context
.global _load_process_context
.cpu cortex-m4
.syntax unified
.thumb

dispatch_task:
	mrs ip, psr
  	push {R4-R11, ip,lr}
	msr psp, r0
	bkpt
	mov r0, #3
	msr control, r0
	// pop {pc}
	// bkpt
	pop {R4-R11}
	push #0xFFF
	bkpt
	bx lr

get_current_msp:
	mrs r0, msp
	bx lr

get_current_psp:
	mrs r0, psp
	bx lr

_save_process_context:
	mrs r0, psp
	stmdb r0!, {r4-r11}
	msr psp, r0

_load_process_context:
	mrs r0, psp
	ldmfd r0!, {r4-r11}
	msr psp, r0

_write_psp:
	msr psp, r0