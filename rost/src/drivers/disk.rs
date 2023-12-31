#![allow(dead_code)]

// SHOULD WORK

use x86_64::instructions::port::Port;
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Debug)]

struct Diskstate {
    pub start: u32,
    pub end: u32
}

lazy_static! {
    static ref DISK_STATE : Mutex<Diskstate> = Mutex::new(
        Diskstate {
            start:0,
            end:0,
        }
    );
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Status {
    Bsy  = 0x80,
    Rdy  = 0x40,
    Drq  = 0x08,
    Df   = 0x20,
    Err  = 0x01,
}

pub fn is_ok(raw: u32) {
    assert!(!(raw >= get_sectors_count() && DISK_STATE.lock().end != 0), "Disk error : sector too big : {} (max {})", raw, get_sectors_count()-1);
}

pub fn write_sector(raw_sector_nb: u32, data: [u32; 128]) {
    is_ok(raw_sector_nb);
    let sector_nb = raw_sector_nb + DISK_STATE.lock().start;
    ata_wait_bzy();
    ata_wait_rdy();
    unsafe {
        Port::new(0x1F6).write(0xE0 | ((sector_nb >> 24) & 0x0F) as u8);
        Port::new(0x1F1).write(0x00_u8);
        Port::new(0x1F2).write(0x01_u8);
        Port::new(0x1F3).write(sector_nb as u8);
        Port::new(0x1F4).write((sector_nb >> 8) as u8);
        Port::new(0x1F5).write((sector_nb >> 16) as u8);
        Port::new(0x1F7).write(0x30_u8);
    }
    ata_wait_bzy();
    ata_wait_rdy(); // strange
    for item in data {
        unsafe {
            Port::new(0x1F0).write(item);
        }
    }
}

pub fn read_sector(raw_sector_nb: u32) -> [u32; 128] {
    is_ok(raw_sector_nb);
    let sector_nb = raw_sector_nb + DISK_STATE.lock().start;
    let mut data = [0; 128];
    ata_wait_bzy();
    ata_wait_rdy();
    unsafe {
        Port::new(0x1F6).write(0xE0 | ((sector_nb >> 24) & 0x0F) as u8);
        Port::new(0x1F1).write(0x00_u8);
        Port::new(0x1F2).write(0x01_u8);
        Port::new(0x1F3).write(sector_nb as u8);
        Port::new(0x1F4).write((sector_nb >> 8) as u8);
        Port::new(0x1F5).write((sector_nb >> 16) as u8);
        Port::new(0x1F7).write(0x20_u8);
    }
    ata_wait_bzy();
    ata_wait_rdy(); // strange
    for item in &mut data {
        unsafe {
            *item = Port::new(0x1F0).read();
        }
    }
    data
}

pub fn get_sectors_count() -> u32 {
    let disk = DISK_STATE.lock();
    disk.end - disk.start
}

fn ata_wait_bzy() {
    let mut port: x86_64::instructions::port::PortGeneric<u8, x86_64::instructions::port::ReadWriteAccess> = Port::new(0x1F7);
    while unsafe { port.read() } & (Status::Bsy as u8) != 0 {
        // Wait until the BSY bit is cleared
    }
}

fn ata_wait_rdy() {
    let mut port: x86_64::instructions::port::PortGeneric<u8, x86_64::instructions::port::ReadWriteAccess> = Port::new(0x1F7);
    while unsafe { port.read() } & (Status::Rdy as u8) == 0 {
        // Wait until the RDY bit is set
    }
}

pub fn init() {
    #[cfg(nodisk)]
    return;
    // lets set DISKSTATE
    // we loop on the disk until wi find a sector full of @
    let mut i = 0;
    let mut data = read_sector(i);
    while data != [64 + (64 << 8) + (64 << 16) + (64 << 24); 128] {
        i += 1;
        data = read_sector(i);
    }
    let start = i+1; // +1 because we want to start after the first void sector
    // now we loop until we find a sector full of @ again
    i += 1;
    data = read_sector(i);
    while data != [64 + (64 << 8) + (64 << 16) + (64 << 24); 128] {
        i += 1;
        data = read_sector(i);
    }
    let mut disk_state = DISK_STATE.lock();
    disk_state.start = start;
    disk_state.end = i;
}
