#![no_std]
#![feature(asm)]

extern crate volatile;

pub mod gpio;
pub mod timer;
pub mod mailbox;
pub mod mini_uart;
pub mod interrupt;

pub const IO_BASE: usize = 0xFFFF_0000_3F00_0000;
