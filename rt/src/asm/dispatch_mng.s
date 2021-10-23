.global __get_current_msp
.global __get_current_psp
.global __save_process_context
.global __load_process_context
.global __write_psp
.global __exec
.cpu cortex-m4
.syntax unified
.thumb


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


__write_psp:
	msr psp, r0
	bx lr

__exec:
	pop {r0, r1}
	mov r1, #0xFFFFFFFD
	str r1, [sp, #8]
	push {r1}
	push {r0}
	// bkpt
