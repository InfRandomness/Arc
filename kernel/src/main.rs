#![no_std]
#![no_main]

use arch::{hal, Hal};
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

static HELLO: &[u8] = b"Hello world!";

entry_point!(kmain);

fn kmain(boot_info: &'static mut BootInfo) -> ! {

    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        for byte in framebuffer.buffer_mut() {
            *byte = 0x90;
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
