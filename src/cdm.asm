.section .text

.global abort
.set abort, halt

.global halt
halt:
halt

.global disable_int
disable_int:
ldps r0
di
shr r0, 8
shr r0, 7
rts

.global restore_int
restore_int:
tst r0
bz zero
ldi r0, 0x8000
zero:
stps r0
rts

.global memcpy
memcpy:
tst r2
blt memcpy_end
memcpy_beg:
ldb r1, r3
stb r0, r3
inc r0
inc r1
dec r2
bge memcpy_beg
memcpy_end:
rts

.global memmove
memmove:
tst r2
ble memmove_end
cmp r0, r1
blt memmove_fwd
bgt memmove_bwd
memmove_end:
rts

memmove_fwd:
memmove_fwd_beg:
ldb r1, r3
stb r0, r3
inc r0
inc r1
dec r2
bge memmove_fwd_beg
rts

memmove_bwd:
add r0, r2, r0
add r1, r2, r1
dec r0
dec r1
memmove_bwd_beg:
ldb r1, r3
stb r0, r3
dec r0
dec r1
dec r2
bge memmove_bwd_beg
rts

.global memset
memset:
tst r2
blt memset_end
memset_beg:
stb r0, r1
inc r0
dec r2
bge memset_beg
memset_end:
rts

.global __mulsi3
__mulsi3:
push r4
push r5
push r6
ldi r4, 0
ldi r5, 0
or r2, r3, r6
bz __mulsi3_end
__mulsi3_loop_body:
shr r3
rcr r2
bcc __mulsi3_skip_add
add r0, r4, r4
addc r1, r5, r5
__mulsi3_skip_add:
shl r0
rcl r1
or r2, r3, r6
bnz __mulsi3_loop_body
__mulsi3_end:
move r4, r0
move r5, r1
pop r6
pop r5
pop r4
rts

.global __mulhi3
__mulhi3:
ldi r2, 0
tst r1
bz __mulhi3_end
__mulhi3_loop_body:
shr r1
bcc __mulhi3_skip_add
add r0, r2
__mulhi3_skip_add:
shl r0
tst r1
bnz __mulhi3_loop_body
__mulhi3_end:
move r2, r0
rts

.global __udivhi3
__udivhi3:
# TODO: WORKAROUND!!! THIS SHOULD BE IMPLEMENTED!!!
rts
