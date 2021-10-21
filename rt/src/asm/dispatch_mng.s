.global dispatch_task
.global __get_current_msp
.global __get_current_psp
.global __save_process_context
.global __load_process_context
.global __write_psp
.cpu cortex-m4
.syntax unified
.thumb

dispatch_task:
	mrs ip, psr
  	push {R4-R11, ip,lr}
	msr psp, r0
	mov r0, #3
	msr control, r0
	// pop {pc}
	// bkpt
	pop {R4-R11}
	push #0xFFF
	bx lr

__get_current_msp:
	mrs r0, msp
	bx lr

__get_current_psp:
	mrs r0, psp
	bx lr

__save_process_context:
	mrs r0, psp
	stmdb r0!, {r4-r11}
	msr psp, r0

__load_process_context:
	mrs r0, psp
	bkpt
	ldmfd r0!, {r4-r11}
	bkpt
	msr psp, r0

__write_psp:
	msr psp, r0
	bkpt
	bx lr