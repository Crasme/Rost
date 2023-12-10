use crate::drivers;
use crate::libs;
use crate::apps;

use x86_64::{
    instructions::interrupts,
    VirtAddr,
};
use bootloader::BootInfo;

use crate::{
    print, println
};

pub fn init(_boot_info: &'static BootInfo) {
    // init of drivers
    print!("Initialising disk... ");
    interrupts::without_interrupts(|| {
        drivers::disk::init();
    });
    println!("[OK] ({} sectors)", drivers::disk::get_sectors_count());

    // init of libs
    print!("Loading interrupt table... ");
    libs::interrupts::init_idt();
    println!("[OK]");
    print!("Loading global descriptor table... ");
    libs::gdt::init();
    println!("[OK]");
    print!("Loading interrupts... ");
    unsafe { libs::interrupts::PICS.lock().initialize() }; 
    println!("[OK]");
    print!("Enabling interrupts... ");
    x86_64::instructions::interrupts::enable();
    println!("[OK]");
    print!("Enabling the filesystem... ");
    libs::filesystem::init();
    println!("[OK]");

    // init of apps
    print!("Starting the shell...");
    apps::shell::init();
    // pas de message OK, on ne veut pas polluer le prompt
}

