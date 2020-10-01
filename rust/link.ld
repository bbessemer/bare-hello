/*
 * Copyright (C) 2020 Brent Bessemer. All rights reserved.
 * Licensed under MIT license.
 */

ENTRY(_start)

SECTIONS
{
    . = 0x100000;
    .text :
    {
        KEEP(*(.multiboot))
        *(.text)
    }
    .data ALIGN(4096) :
    {
        *(.rodata*)
        *(.data)
    }
}
