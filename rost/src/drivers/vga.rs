#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub fn write_char(adress: usize, content: u8, color: Color) {
    let vga_adress = 0xb8000 as *mut u8;
    unsafe {
        *vga_adress.offset(adress as isize * 2) = content;
        *vga_adress.offset(adress as isize * 2 + 1) = color as u8;
    }
}

pub fn print(string: &[u8], color: Color) {
    for (i, &byte) in string.iter().enumerate() {
        write_char(
            i, 
            byte,
            color
        );
    } 
}
