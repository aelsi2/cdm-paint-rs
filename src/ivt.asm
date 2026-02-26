.section .ivt, "a", @progbits

.short main, 0b1000000000000000
.short on_exception, 0
.short on_exception, 0
.short on_exception, 0
.short on_exception, 0
.short on_input, 0
.short on_timer, 0
