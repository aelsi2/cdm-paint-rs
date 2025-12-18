rsect cdm.asm

abort>
halt>
halt

disable_int>
ldps r0
di
shr r0, 8
shr r0, 7
rts

restore_int>
tst r0
bz zero
ldi r0, 0x8000
zero:
ldps r1
or r1, r0, r1
stps r0
rts

memcpy>
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

__mulhi3>
ldi r2, 0
while
tst r1
stays nz
    if
        shr r1
    is cs
        add r0,r2
    fi
    shl r0
wend
move r2,r0
rts

end.
