#![no_std]

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::Hal as hal;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use self::aarch64::Hal as Hal;

pub trait Hal {
    fn hlt();
    fn wait(n: usize);
    fn irq_enable(enable: bool);
}