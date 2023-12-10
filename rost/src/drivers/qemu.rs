#![allow(dead_code)]

use core::arch::asm;
use crate::libs;

pub fn exit_qemu() {
    // TODO : fix
    use x86_64::instructions::port::Port;

    unsafe {
        Port::new(0x604).write(0x2000 as u32);  // QEMU
        Port::new(0xB004).write(0x2000 as u32); // BOCHS
        Port::new(0x4004).write(0x3400 as u32); // VIRTUALBOX
    }

    // we stop interrupts
    unsafe {
        asm!("cli");
        asm!("hlt");
    }
}

pub fn restart_qemu() {
    use x86_64::instructions::port::Port;

    let mut good = 0x02;

    let mut port: x86_64::instructions::port::PortGeneric<u8, x86_64::instructions::port::ReadWriteAccess> = Port::new(0x64);
    
    while (good & 0x02) != 0 {
        unsafe {
            good = port.read();
        }
    }

    unsafe {
        Port::new(0x64).write(0xFE as u32)
    }

    libs::general::hlt_loop();
}
