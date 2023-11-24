#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod drivers;

#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    drivers::qemu::exit_qemu(drivers::qemu::QemuExitCode::Failed);
    loop {}
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) { // new
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run(); // new
    }
    drivers::qemu::exit_qemu(drivers::qemu::QemuExitCode::Success);
}

// OTHER

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Boot complete");

    #[cfg(test)]
    test_main();

    loop {}
}
