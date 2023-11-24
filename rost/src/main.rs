#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
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
    loop {}
}

// OTHER

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Boot complete");

    // Lance les tests
    #[cfg(test)]
    test_main();

    loop {}
}
