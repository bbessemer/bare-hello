/*
 * Copyright (C) 2020 Brent Bessemer. All rights reserved.
 * Licensed under MIT license.
 */

#define MB_MAGIC    0x1badb002
#define MB_FLAGS    ((1 << 0) | (1 << 1))
#define MB_CHECKSUM -(MB_MAGIC + MB_FLAGS)

struct mbhdr
{
    int magic;
    int flags;
    int checksum;
};

__attribute__((section(".multiboot")))
struct mbhdr multiboot_header = { MB_MAGIC, MB_FLAGS, MB_CHECKSUM };

/* Just below the ISA memory hole */
#define STACK_TOP   0xf00000

static void vga_print(const char *, short);

__attribute__((naked))
void _start(void)
{
    asm volatile("mov %0, %%esp;"
                 "jmp kmain"
                 : : "i"(STACK_TOP));
}

__attribute__((noreturn))
void kmain(void)
{
    vga_print("Hello, world!", 0x07);

    while (1) {
        asm volatile("hlt");
    }
}

#define VGA ((short *)0xb8000)
#define VGA_SIZE (80*25)

static void vga_print(const char *msg, short color)
{
    int i = 0;
    for (; i < VGA_SIZE && msg[i]; i++)
        VGA[i] = msg[i] | (color << 8);
    for (; i < VGA_SIZE; i++)
        VGA[i] = color << 8;
}
