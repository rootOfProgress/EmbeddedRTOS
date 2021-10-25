.global __get_current_msp
.global __get_current_psp
.global __save_process_context
.global __load_process_context
.global __write_psp
.global __exec
.global __get_msp_entry
.global __set_exec_mode
.cpu cortex-m4
.syntax unified
.thumb


__get_current_msp:
	mrs r0, msp
	bx lr

__get_current_psp:
	mrs r0, psp
	bx lr

__get_msp_entry:
	mov r0, r7
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

__set_exec_mode:
	mov r2, r0
	bx lr

__exec:
	// bkpt
	// sub r4, 
	mrs r3, msp
	sub r4, r0, r3
	add r4, #0x04
	// bkpt
	mov r1, r2
	str r1, [sp, r4]
	// bkpt
