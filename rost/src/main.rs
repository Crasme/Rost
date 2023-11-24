#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]

#![test_runner(crate::libs::tests::test_runner)]

#![reexport_test_harness_main = "test_main"]

#[allow(unused_imports)]
use core::panic::PanicInfo;

mod drivers;
mod libs;

#[cfg(not(test))] // handler normal
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    libs::general::hlt_loop();
}

// OTHER

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Boot complete");

    libs::init(); // doit être au dessus des tests

    // Lance les tests
    #[cfg(test)]
    test_main();

    drivers::disk::write_sector(1, [0 as u32; 128]);

    println!("Ecrit !");

    loop {}

    libs::general::hlt_loop();
}
