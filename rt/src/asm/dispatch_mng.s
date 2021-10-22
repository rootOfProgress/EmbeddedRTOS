.global dispatch_task
.global __get_current_msp
.global __get_current_psp
.global __save_process_context
.global __load_process_context
.global __write_psp
.global __exec
.global __msp_workaround
// .global __foo
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
	bx lr

__load_process_context:
	mrs r0, psp
	ldmfd r0!, {r4-r11}
	msr psp, r0
	bx lr

__msp_workaround:
	pop {r0, r1}
	bx lr

__write_psp:
	msr psp, r0
	bx lr
__exec:
	mov lr, #0xFFFFFFFD
	// mov r0, #0xFFFFFFFD
	add sp, #8
	// str r0, [sp], #-8
	// bx lr
