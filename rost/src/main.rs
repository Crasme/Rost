#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod drivers;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // TODO : show message error
    drivers::vga::print(
        b"Fatal Error", 
        drivers::vga::Color::Red
    );

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    drivers::vga::print(
        b"coucou !", 
        drivers::vga::Color::Red
    );

    loop {}
}
