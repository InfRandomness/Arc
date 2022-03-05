#![feature(generic_associated_types)]
#![no_std]

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::Hal as hal;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use self::aarch64::Hal;

pub trait Hal {
    fn hlt();
    fn wait(n: usize);
    fn irq_enable(enable: bool);
    fn without_interrupts<F, R>(f: F) -> R
    where
        F: FnOnce() -> R;
    fn port<F>(port: u16) -> Self::Port<F>;
    type Port<F>;
}
