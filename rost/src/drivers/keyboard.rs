use x86_64::instructions::port::Port;
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use spin::Mutex;

use crate::libs::interrupts::KEYBOARD;
pub struct Handlers {
    handlers: [Option<fn(DecodedKey)>; 10],
    num_handlers: usize,
}

impl Handlers {
    pub fn new() -> Self {
        Self {
            handlers: [None; 10],
            num_handlers: 0,
        }
    }

    pub fn add_handler(&mut self, handler: fn(DecodedKey)) {
        if self.num_handlers < 10 {
            self.handlers[self.num_handlers] = Some(handler);
            self.num_handlers += 1;
        } else {
            panic!("Too many keyboard handlers");
        }
    }

    #[allow(dead_code)] // TODO : remove
    pub fn remove_handler(&mut self, handler: fn(DecodedKey)) {
        let mut i = 0;
        while i < self.num_handlers {
            if self.handlers[i] == Some(handler) {
                self.handlers[i] = None;
                self.num_handlers -= 1;
                break;
            }
            i += 1;
        }
        assert!(i != self.num_handlers, "Handler not found"); 
    }

    pub fn run_handlers(&self, key: DecodedKey) {
        for i in 0..self.num_handlers {
            if let Some(handler) = self.handlers[i] {
                handler(key);
            }
        }
    }
}

// array of handlers that take a decodedkey and return nothing (10 max)
lazy_static! {
    pub static ref HANDLERS: Mutex<Handlers> = Mutex::new(Handlers::new());
}

pub fn run_handlers(keyboard: &KEYBOARD) {
    let mut keyboard_l = keyboard.lock();
    let mut port = Port::new(0x60);
    
    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard_l.add_byte(scancode) {
        if let Some(key) = keyboard_l.process_keyevent(key_event) {
            HANDLERS.lock().run_handlers(key);
        }
    }
}
