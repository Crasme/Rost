#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod drivers;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO : show message error
    println!("Erreur fatale.");
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Boot complete");
    loop {}
}
