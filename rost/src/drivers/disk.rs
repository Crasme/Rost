#![allow(dead_code)]

// BROKEN FOR NOW

use x86_64::instructions::port::Port;

use crate::print;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum STATUS {
    StatusBsy  = 0x80,
    StatusRdy  = 0x40,
    StatusDrq  = 0x08,
    StatusDf   = 0x20,
    StatusErr  = 0x01,
}

pub fn write_sector(sector_nb: u32, data: [u32; 128]) {
    print!(".");
    ata_wait_bzy();
    print!(".");
    ata_wait_drq();
    print!(".");
    unsafe {
        Port::new(0x1F6).write(0xE0 | ((sector_nb >> 24) & 0x0F) as u8);
        Port::new(0x1F1).write(0x00 as u8);
        Port::new(0x1F2).write(0x01 as u8);
        Port::new(0x1F3).write(sector_nb as u8);
        Port::new(0x1F4).write((sector_nb >> 8) as u8);
        Port::new(0x1F5,).write((sector_nb >> 16) as u8);
        Port::new(0x1F7).write(0x30 as u8);
    }
    print!(".");
    ata_wait_bzy();
    print!(".");
    ata_wait_drq();
    print!(".");
    for i in 0..128 {
        unsafe {
            Port::new(0x1F0).write(data[i] as u32);
        }
    }
}

pub fn read_sector(sector_nb: u32) -> [u32; 128] {
    let mut data = [0; 128];
    ata_wait_bzy();
    ata_wait_drq();
    unsafe {
        Port::new(0x1F6).write(0xE0 | ((sector_nb >> 24) & 0x0F) as u8);
        Port::new(0x1F1).write(0x00 as u8);
        Port::new(0x1F2).write(0x01 as u8);
        Port::new(0x1F3).write(sector_nb as u8);
        Port::new(0x1F4).write((sector_nb >> 8) as u8);
        Port::new(0x1F5).write((sector_nb >> 16) as u8);
        Port::new(0x1F7).write(0x20 as u8);
    }
    ata_wait_bzy();
    ata_wait_drq();
    for i in 0..128 {
        unsafe {
            data[i] = Port::new(0x1F0).read();
        }
    }
    data
}

pub fn get_sectors_count() -> u32 {
    // return 0 if ata is not present
    for _ in 0..0x1000 {
        let mut port: x86_64::instructions::port::PortGeneric<u8, x86_64::instructions::port::ReadWriteAccess> = Port::new(0x1F7);
        if unsafe {port.read() as u8} & (STATUS::StatusBsy as u8) != 0 {
            return 0;
        }
    }
    unsafe {
        Port::new(0x1F6).write((0xE0 | ((0 >> 24) & 0xF)) as u8);
        Port::new(0x1F2).write(0 as u8);
        Port::new(0x1F3).write(0 as u8);
        Port::new(0x1F4).write(0 as u8);
        Port::new(0x1F5).write(0 as u8);
        Port::new(0x1F7).write(0xEC as u8); // send the identify command
    }

    let mut port: x86_64::instructions::port::PortGeneric<u16, x86_64::instructions::port::ReadWriteAccess> = Port::new(0x1F7);
    if unsafe { port.read() } == 0 {
        return 0;
    }

    ata_wait_bzy();
    ata_wait_drq();

    let mut bytes: [u16; 256] = [0; 256];
    for i in 0..256 {
        let mut port: x86_64::instructions::port::PortGeneric<u16, x86_64::instructions::port::ReadWriteAccess> = Port::new(0x1F0);
        bytes[i] = unsafe { port.read() };
    }

    let size: u32 = ((bytes[61] as u32) << 16) | (bytes[60] as u32);
    size
}

fn ata_wait_bzy() {
    let mut port: x86_64::instructions::port::PortGeneric<u8, x86_64::instructions::port::ReadWriteAccess> = Port::new(0x1F7);
    while unsafe { port.read() } & (STATUS::StatusBsy as u8) != 0 {
        // Wait until the BSY bit is cleared
    }
}

fn ata_wait_drq() {
    let mut port: x86_64::instructions::port::PortGeneric<u8, x86_64::instructions::port::ReadWriteAccess> = Port::new(0x1F7);
    while unsafe { port.read() } & (STATUS::StatusDrq as u8) != 0 {
        // Wait until the DRQ bit is cleared
    }
}
