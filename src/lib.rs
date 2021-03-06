#![feature(const_fn)]
#![feature(asm)]
#![no_std]
#![cfg_attr(test, allow(unused_features))]

#![crate_name = "x86"]
#![crate_type = "lib"]

#[macro_use]
mod bitflags;

#[macro_use]
extern crate raw_cpuid;

#[cfg(feature = "performance-counter")]
#[macro_use]
extern crate phf;

mod std {
    pub use core::fmt;
    pub use core::ops;
    pub use core::option;
}

macro_rules! bit {
    ( $x:expr ) => {
        1 << $x
    };
}

macro_rules! check_flag {
    ($doc:meta, $fun:ident, $flag:ident) => (
        #[$doc]
        pub fn $fun(&self) -> bool {
            self.contains($flag)
        }
    )
}

macro_rules! is_bit_set {
    ($field:expr, $bit:expr) => (
        $field & (1 << $bit) > 0
    )
}

macro_rules! check_bit_fn {
    ($doc:meta, $fun:ident, $field:ident, $bit:expr) => (
        #[$doc]
        pub fn $fun(&self) -> bool {
            is_bit_set!(self.$field, $bit)
        }
    )
}

pub mod io;
pub mod controlregs;
pub mod msr;
pub mod time;
pub mod irq;
pub mod rflags;
pub mod paging;
pub mod segmentation;
pub mod task;
pub mod dtables;
pub mod syscall;
pub mod sgx;
#[cfg(feature = "performance-counter")]
pub mod perfcnt;
pub mod cpuid {
    pub use raw_cpuid::*;
}
pub mod tlb;
