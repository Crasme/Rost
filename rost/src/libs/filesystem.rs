use crate::drivers::disk as disk;

use crate::print;

#[allow(unreachable_code)]
pub fn init() {
    #[cfg(nodisk)]
    return;
    let buffer: [u32; 128] = disk::read_sector(0);
    // if the first u32 isnt our magic number, we raise an error
    // print buffer[0] as hexa
    print!("{:#04x} ", buffer[0]);
    if buffer[0] != 0xdeadbeef {
        panic!("Filesystem corrupted");
    }
}
