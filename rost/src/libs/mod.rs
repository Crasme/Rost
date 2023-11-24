pub mod tests;
pub mod interrupts;
pub mod gdt;
pub mod general;

use crate::{print, println};

pub fn init() {
    print!("Loading interrupt table... ");
    interrupts::init_idt();
    println!("[OK]");
    print!("Loading global descriptor table... ");
    gdt::init();
    println!("[OK]");
    print!("Loading interrupts... ");
    unsafe { interrupts::PICS.lock().initialize() }; 
    println!("[OK]");
    print!("Enabling interrupts... ");
    x86_64::instructions::interrupts::enable();
    println!("[OK]");
}

