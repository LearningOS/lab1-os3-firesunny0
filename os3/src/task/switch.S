/*
 * @Author: firesunny
 * @Date: 2022-11-26 20:19:20
 * @LastEditTime: 2022-11-26 20:36:48
 * @FilePath: /lab1-os3-firesunny0/os3/src/task/switch.S
 * @Description: 
 */

.altmacro
.macro SAVE_SN n
	sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
	ld s\n, (\n+2)*8(a1)
.endm
	.section .text
	.globl __switch
__switch:
	# save kernel stack
	sd sp, 8(a0)
	# save ra
	sd ra, 0(a0)
	# save s0-s11
	.set n, 0
	.rept 12
		SAVE_SN %n
		.set n, n+1
	.endr
	# restore 
	.set n, 0
	.rept 12
		LOAD_SN %n
		.set n, n+1
	.endr
	ld ra, 0(a1)
	ld sp, 8(a1)
	// ret (sie, sp)
	ret
	