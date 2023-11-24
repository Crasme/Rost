use x86_64::instructions::port::Port;
use pc_keyboard::DecodedKey;

use crate::libs::interrupts::KEYBOARD;
use crate::print;

pub fn run_scancode(keyboard: &KEYBOARD) {
    let mut keyboard_l = keyboard.lock();
    let mut port = Port::new(0x60);
    
    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard_l.add_byte(scancode) {
        if let Some(key) = keyboard_l.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
}
