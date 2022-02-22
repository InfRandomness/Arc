use x86_64;

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
}