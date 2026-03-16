.section .text

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
