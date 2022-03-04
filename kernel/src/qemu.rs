use arch::{hal, Hal};

pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    unsafe {
        hal::port(0xf4).write(exit_code as u32);
    }

    loop {
        hal::hlt()
    }
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}
