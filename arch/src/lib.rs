#![no_std]

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::Hal as hal;
use ::x86_64::instructions::port::{PortGeneric, ReadWriteAccess};

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
    // TODO: Remove any usage of x86_64 specific variables here.
    fn port<F>(port: u16) -> PortGeneric<F, ReadWriteAccess>;
}
