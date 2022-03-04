use x86_64;
use x86_64::instructions::port::{PortGeneric, ReadWriteAccess};

pub struct Hal;

impl super::Hal for Hal {
    fn hlt() {
        x86_64::instructions::hlt();
    }

    fn wait(n: usize) {
        todo!()
    }

    fn irq_enable(enable: bool) {
        todo!()
    }

    fn without_interrupts<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        x86_64::instructions::interrupts::without_interrupts(f)
    }

    fn port<F>(port: u16) -> PortGeneric<F, ReadWriteAccess> {
        PortGeneric::new(port)
    }
}
