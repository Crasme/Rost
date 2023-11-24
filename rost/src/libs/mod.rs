pub mod tests;
pub mod interrupts;
pub mod gdt;

use crate::{print, println};

pub fn init() {
    print!("Loading interrupt table... ");
    interrupts::init_idt();
    println!("[OK]");
    print!("Loading global descriptor table... ");
    gdt::init();
    println!("[OK]");
}

