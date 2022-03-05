use x86_64;
use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};

pub struct Hal;

impl super::Hal for Hal {
    fn hlt() {
        x86_64::instructions::hlt();
    }

    fn wait(_n: usize) {
        todo!()
    }

    fn irq_enable(_enable: bool) {
        todo!()
    }

    fn without_interrupts<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        x86_64::instructions::interrupts::without_interrupts(f)
    }

    fn port<F>(port: u16) -> Port<F> {
        PortGeneric::new(port)
    }

    type Port<F> = PortGeneric<F, ReadWriteAccess>;
}
