pub mod vga;
pub mod serial;
pub mod qemu;
pub mod keyboard;
pub mod disk;

use x86_64::instructions::interrupts;

use crate::{print, println, drivers};

pub fn init() {
    interrupts::without_interrupts(|| {
        print!("Initialising disk... ");
        disk::init();
        println!("[OK] ({} sectors)", drivers::disk::get_sectors_count());
    })
}
