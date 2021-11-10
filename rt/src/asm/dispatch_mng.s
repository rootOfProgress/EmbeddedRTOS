.global __get_current_psp
.global __save_process_context
.global __load_process_context
.global __write_psp
.global __set_exc_return
.global __instant_start
.global __trap
.cpu cortex-m4
.syntax unified
.thumb

__get_current_psp:
	mrs r0, psp
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

__set_exc_return:
	mrs r0, msp
	sub r4, r7, r0
	add r4, #0x04
	mov r1, #0xfffffffd
	str r1, [sp, r4]
	bx lr

