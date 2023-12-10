use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;

use spin::Mutex;

use crate::print;

use crate::drivers;
use crate::drivers::keyboard;
use crate::println;

use alloc::boxed::Box;

// init of a buffer (list of chars) and a cursor
pub struct Writer {
    content: [char; drivers::vga::BUFFER_WIDTH],
    cursor_pos: usize,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            content: ['\0'; drivers::vga::BUFFER_WIDTH],
            cursor_pos: 0,
        }
    }

    pub fn write_char(&mut self, c: char) {
        if self.cursor_pos == drivers::vga::BUFFER_WIDTH {
            // we do not want to write outside of the buffer
        }
        self.content[self.cursor_pos] = c;
        self.cursor_pos += 1;
        if self.cursor_pos == drivers::vga::BUFFER_WIDTH {
            self.cursor_pos = 0;
        }
    }

    pub fn remove(&mut self) {
        if self.cursor_pos == 0 {
            // we do not want to remove outside of the buffer
            return;
        }
        self.cursor_pos -= 1;
        self.content[self.cursor_pos] = '\0';
        // we print a space in the vga buffer
        drivers::vga::remove_char();
    }

    pub fn reset(&mut self) {
        self.cursor_pos = 0;
        self.content = ['\0'; drivers::vga::BUFFER_WIDTH];
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

fn print_buffer(buffer: [char; drivers::vga::BUFFER_WIDTH]) {
    for c in buffer.iter() {
        if *c == '\0' {
            return;
        }
        print!("{}", c);
    }
}

fn is_the_same(command: [char; drivers::vga::BUFFER_WIDTH], raw_command: &str) -> bool {
    for (i, c) in raw_command.chars().enumerate() {
        if command[i] != c {
            return false;
        }
    }
    return true;
}

// TO ADD : https://pubs.opengroup.org/onlinepubs/9699919799/idx/utilities.html
fn run_command(command: [char; drivers::vga::BUFFER_WIDTH]) {
    // we print a new line
    print!("\n");
    // check if command is a known command
    if is_the_same(command, "help") {
        print!("Available commands :\n");
        print!("help : show this help\n");
        print!("clear : clear the screen\n");
    } else if is_the_same(command, "clear") {
        drivers::vga::clear_screen();
    } else if is_the_same(command, "stop") {
        drivers::qemu::exit_qemu();
    } else if is_the_same(command, "reboot") {
        drivers::qemu::restart_qemu();
    } else if is_the_same(command, "test") {
        let x = Box::new(41);
        let y = Box::new(42);
        println!("{:?}", *x);
        println!("{:?}", *y);
    } else {
        print!("Unknown command : ");
        print_buffer(command);
        print!("\n");
    }
    // we show the prompt
    print!("> ");
}

fn run_key(key: DecodedKey) {
    let mut writer = WRITER.lock();
    match key {
        DecodedKey::Unicode('\n') => {
            writer.cursor_pos = 0;
            run_command(writer.content);
            writer.reset();
        },
        // if backspace
        DecodedKey::Unicode('\u{8}') => {
            writer.remove();
        }
        DecodedKey::Unicode(key) => {
            // ie the key is a printable char
            if key.is_ascii() {
                writer.write_char(key);
                print!("{}", key);
            }
        }
        DecodedKey::RawKey(_key) => {

        },
    }
}

pub fn init() {
    keyboard::HANDLERS.lock().add_handler(run_key);
    // we show the prompt
    print!("\n> ");
}
