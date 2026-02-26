.section .mmio, "aw", @nobits

display_row:
.space 4  #0x80
display_row_index:
.space 1  #0x84
timer_state:
.space 1  #0x85
cursor1_position:
.space 2  #0x86
cursor2_position:
.space 2  #0x88
menu_data:
.space 1  #0x8a
menu_cursor_position:
.space 1  #0x8b
.global input_state
input_state:
.space 1  #0x8c

.section .text

.global timer_enable
timer_enable:
ldi r0, 1
ldi r1, timer_state
stb r1, r0
rts

.global timer_disable
timer_disable:
ldi r0, 0
ldi r1, timer_state
stb r1, r0
rts

# r0: row index
.global menu_set_cursor
menu_set_cursor:
ldi r1, menu_cursor_position
stb r1, r0
rts

# r0: menu data
.global menu_set_data
menu_set_data:
ldi r1, menu_data
stb r1, r0
rts

# r0: pixel index in row major order
.global display_set_primary_cursor
display_set_primary_cursor:
ldi r1, cursor1_position
stw r1, r0
rts

# r0: pixel index in row major order
.global display_set_secondary_cursor
display_set_secondary_cursor:
ldi r1, cursor2_position
stw r1, r0
rts

# r0: buffer address, r1: start row index, r2: end row index
.global display_write_range
display_write_range:
push r4
push r5
add r1, r0
add r1, r0
add r1, r0
add r1, r0
ldi r3, display_row_index
ldi r4, display_row
br while_cond
while_begin:
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
while_cond:
cmp r1, r2
ble while_begin
pop r5
pop r4
rts
