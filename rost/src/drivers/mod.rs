pub mod vga;
pub mod serial;
pub mod qemu;
pub mod keyboard;
pub mod disk;

use crate::{print, println, drivers};

pub fn init() {
    print!("Initialising disk... ");
    disk::init();
    println!("[OK] ({} sectors)", drivers::disk::get_sectors_count());
}
