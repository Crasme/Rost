use crate::drivers::disk as disk;

use crate::print;

pub fn init() {
    let buffer: [u32; 128] = disk::read_sector(0);
    // if the first byte isnt our magic number, we raise an error
    // print buffer[0] as hexa
    print!("{:#04x} ", buffer[0]);
    if buffer[0] != 0xdeadbeef {
        panic!("Filesystem corrupted");
    }
}