//! This is the main file of the rost kernel.
//! It is responsible for initializing the kernel and starting the first process.

#![allow(unused_imports)]
#![deny(clippy::all)]

#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]

extern crate alloc;
use alloc::boxed::Box;

use bootloader::{BootInfo, entry_point};

use core::panic::PanicInfo;

mod drivers;
mod libs;
mod apps;

mod utils;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    libs::general::hlt_loop();
}

entry_point!(main);

/// Entry point of the kernel
#[no_mangle]
fn main(boot_info: &'static BootInfo) -> ! {
    use x86_64::{structures::paging::{Translate, Page}, VirtAddr};

    println!("Boot complete");
    utils::init(boot_info);

    //let _x = Box::new(41);

    libs::general::hlt_loop();
}

