asect 0x80

display_row:
ds 4  #0x80
display_row_index:
ds 1  #0x84
timer_state:
ds 1  #0x85
cursor1_position:
ds 2  #0x86
cursor2_position:
ds 2  #0x88
menu_data:
ds 1  #0x8a
menu_cursor_position:
ds 1  #0x8b
input_state>
ds 1  #0x8c

rsect io_funcs

timer_enable>
ldi r0, 1
ldi r1, timer_state
stb r1, r0
rts

timer_disable>
ldi r0, 0
ldi r1, timer_state
stb r1, r0
rts

# r0: row index
menu_set_cursor>
ldi r1, menu_cursor_position
stb r1, r0
rts

# r0: menu data
menu_set_data>
ldi r1, menu_data
stb r1, r0
rts

# r0: pixel index in row major order
display_set_primary_cursor>
ldi r1, cursor1_position
stw r1, r0
rts

# r0: pixel index in row major order
display_set_secondary_cursor>
ldi r1, cursor2_position
stw r1, r0
rts

# r0: buffer address, r1: start row index, r2: end row index
display_write_range>
save r4
save r5
add r1, r0
add r1, r0
add r1, r0
add r1, r0
ldi r3, display_row_index
ldi r4, display_row
while
cmp r1, r2
stays le
stb r3, r1
ldw r0, r5
stw r4, r5
add r4, 2
add r0, 2
ldw r0, r5
stw r4, r5
sub r4, 2
add r0, 2
inc r1
wend
restore
restore
rts

end.
