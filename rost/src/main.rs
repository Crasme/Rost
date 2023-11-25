#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod drivers;
mod libs;

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

    let l = [64; 128];
    println!("Ecrit2 !");
    drivers::disk::write_sector(0, l);
    println!("Ecrit !");
    println!("{:?}", drivers::disk::read_sector(0)[0]);

    libs::general::hlt_loop();
}
