// .global __get_current_msp
.global __get_current_psp
.global __save_process_context
.global __load_process_context
.global __write_psp
// .global __exec
// .global __get_msp_entry
.global __save_main_context
// .global __set_exec_mode
// .global	__exec_kernel
.global __set_exc_return
.global __instant_start
.global __trap
.cpu cortex-m4
.syntax unified
.thumb


/* __get_current_msp:
	mrs r0, msp
	bx lr
 */
__get_current_psp:
	mrs r0, psp
	bx lr

/* __get_msp_entry:
	mov r0, r7
	bx lr */

__save_main_context:
	mrs r0, msp
	stmdb r0!, {r4-r11}
	msr psp, r0
	bx lr

__save_process_context:
	mrs r0, psp
	stmdb r0!, {r4-r11}
	msr psp, r0
	bx lr

__load_process_context:
	ldmfd r0!, {r4-r11}
	msr psp, r0
	bx lr

__instant_start:
	bx pc

__trap:
	mov r2, r0
	svc 0
	bx lr

__write_psp:
	msr psp, r0
	bx lr

/* __set_exec_mode:
	mov r2, r0
	bx lr
 */
/* __exec:
	// bkpt
	// sub r4, 
	mrs r3, msp
	sub r4, r0, r3
	add r4, #0x04
	// bkpt
	mov r1, #0xfffffffd
	str r1, [sp, r4]
	// bkpt
	bx lr */

__set_exc_return:
	mrs r0, msp
	sub r4, r7, r0
	add r4, #0x04
	mov r1, #0xfffffffd
	str r1, [sp, r4]
	// bkpt
	bx lr

