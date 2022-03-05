pub struct Hal;

impl super::Hal for Hal {
    fn hlt() {
        todo!()
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
        todo!()
    }

    fn port<F>(port: u16) -> F {
        todo!()
    }
}
