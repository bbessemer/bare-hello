#![no_std]
#![no_main]

#![feature(asm)]
#![feature(naked_functions)]

use core::panic::PanicInfo;
use core::slice;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const MB_MAGIC: i32 = 0x1badb002;
const MB_FLAGS: i32 = (1 << 0) | (1 << 1) | (1 << 16);
const MB_CHECKSUM: i32 = -(MB_MAGIC + MB_FLAGS);

struct MultibootHeader {
    _magic: i32,
    _flags: i32,
    _checksum: i32,
    _header_addr: &'static MultibootHeader,
    _load_addr: u32,
    // This is not a function. It's just the only way Rust has to get the
    // address of an external symbol.
    _load_end_addr: unsafe extern fn(),
    _bss_end_addr: u32,
    _start_addr: unsafe extern fn() -> !,
}

extern {
    fn __end();
}

#[no_mangle]
#[link_section = ".multiboot"]
static multiboot_header: MultibootHeader = MultibootHeader {
    _magic: MB_MAGIC,
    _flags: MB_FLAGS,
    _checksum: MB_CHECKSUM,
    _header_addr: &multiboot_header,
    _load_addr: 0x100000,
    _load_end_addr: __end,
    _bss_end_addr: 0,
    _start_addr: _start,
};

struct Vga {
    buf: &'static mut [u16],
}

impl Vga {
    pub fn new() -> Self {
        Vga {
            buf: unsafe {
                slice::from_raw_parts_mut(0xb8000 as *mut u16, 80*25)
            },
        }
    }

    pub fn print(&mut self, msg: &[u8], color: u16) {
        for (dst, src) in self.buf.iter_mut().zip(msg.iter()) {
            *dst = (*src as u16) | (color << 8);
        }
        for dst in &mut self.buf[msg.len()..] {
            *dst = color << 8;
        }
    }
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start() -> ! {
    asm!("mov esp, {}", const 0xf00000);
    
    let mut vga = Vga::new();
    vga.print(b"Hello, world!", 0x07);

    loop {
        asm!("hlt");
    }
}

// rustc expects this to exist
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, len: usize) {
    for i in 0..len {
        *dest.offset(i as isize) = *src.offset(i as isize);
    }
}
