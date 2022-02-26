#![no_std]
#![no_main]

mod vga;

use arch::{hal, Hal};
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

static HELLO: &[u8] = b"Hello world!";

entry_point!(kmain);

fn kmain(boot_info: &'static mut BootInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {
        hal::hlt()
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        hal::hlt()
    }
}
