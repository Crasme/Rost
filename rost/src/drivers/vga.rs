use lazy_static::lazy_static;
use volatile::Volatile;
use spin::Mutex;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct Couleur(u8);

impl Couleur {
    const fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Char {
    ascii_character: u8,
    color_code: Couleur,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]

struct Buffer {
    chars: [[Volatile<Char>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    colonne: usize,
    couleur: Couleur,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.colonne >= BUFFER_WIDTH {
                    self.new_line();
                }

                let ligne = BUFFER_HEIGHT-1;
                let colonne = self.colonne;

                let couleur = self.couleur;
                self.buffer.chars[ligne][colonne].write(Char {
                    ascii_character: byte,
                    color_code: couleur,
                });
                self.colonne += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe), // osefent
            }

        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.colonne = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Char {
            ascii_character: b' ',
            color_code: self.couleur,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn clear_screen(&mut self) {
        // we clear all rows
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
    }

    fn remove_char(&mut self) {
        if self.colonne == 0 {
            // we do not want to remove outside of the buffer
            return;
        }
        self.colonne -= 1;
        let ligne = BUFFER_HEIGHT-1;
        let colonne = self.colonne;
        let blank = Char {
            ascii_character: b' ',
            color_code: self.couleur,
        };
        self.buffer.chars[ligne][colonne].write(blank);
    }
}

pub fn remove_char() {
    WRITER.lock().remove_char();
}

pub fn clear_screen() {
    WRITER.lock().clear_screen();
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        colonne: 0,
        couleur: Couleur::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
