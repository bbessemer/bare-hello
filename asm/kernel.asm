;;;
; Copyright (C) 2020 Brent Bessemer. All rights reserved.
; Licensed under MIT license.
;;;

bits 32
org 0x100000

MB_MAGIC equ    0x1badb002
MB_FLAGS equ    (1 << 0) | (1 << 1) | (1 << 16)
MB_CHECKSUM equ -(MB_MAGIC + MB_FLAGS)

multiboot_header:
    dd MB_MAGIC
    dd MB_FLAGS
    dd MB_CHECKSUM
    dd multiboot_header
    dd 0x100000
    dd __end
    dd 0
    dd _start

global _start
_start:
    mov esp, 0xf00000   ; Just below ISA hole

    mov edi, 0xb8000
    mov esi, hello
    mov ah, 0x07
    call vga_clear
.halt:
    hlt
    jmp .halt

vga_clear:
    mov ecx, 80*25
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

__end:
