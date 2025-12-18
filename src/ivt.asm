asect 0
main: ext
on_exception: ext
on_timer: ext
on_input: ext

dc main, 0b1000000000000000
dc on_exception, 0
dc on_exception, 0
dc on_exception, 0
dc on_exception, 0
dc on_input, 0
dc on_timer, 0
align 0x80

end.
