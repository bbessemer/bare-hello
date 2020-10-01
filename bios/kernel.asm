;;;
; Copyright (C) 2020 Brent Bessemer. All rights reserved.
; Licensed under MIT license.
;;;

bits 16
org 0x7c00

global _start
_start:
    xor ax, ax
    mov ss, ax
    mov sp, stack_top

    mov si, hello
    mov ah, 0x07
    call vga_print
.halt:
    hlt
    jmp .halt

vga_print:
    mov dx, 0xb800
    mov es, dx
    mov di, 0
    mov cx, 80*25
.print:
    lodsb
    test al, al
    jz .clear
    stosw
    jmp .print
.clear:
    rep stosw
    ret

hello:
    db "Hello, world!", 0

times 510-($-$$) db 0

stack_top:

db 0x55, 0xaa
