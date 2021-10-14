.global bar
.cpu cortex-m4
.syntax unified
.thumb
/*
bar:
  mrs ip, psr
.global bar
.cpu cortex-m4
.thumb
*/
bar:
	mrs ip, psr
  	push {R4-R11, ip,lr}
	msr psp, r0
	mov r0, #3
	msr control, r0
	pop {R4-R11, ip,lr}
	bx lr