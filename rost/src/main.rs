#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod drivers;
mod libs;
mod apps;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    libs::general::hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Boot complete");

    libs::init();
    drivers::init();
    apps::init();

    libs::general::hlt_loop();
}
