extern crate spin;
extern crate cpuio;

use self::cpuio::Port;
use spin::Mutex;
use input::scancode;

static KEYBOARD: Mutex<Port<u8>> = Mutex::new(unsafe {
    Port::new(0x60)
});

pub struct PollingKeyboard {
    callback: fn(char),
    last_value: u8
}

impl PollingKeyboard {
    pub fn new(callback: fn(char)) -> PollingKeyboard {
        PollingKeyboard {
            callback: callback,
            last_value: 250 // Seems to be default value?
        }
    }

    pub fn update(&mut self) {
        let code = KEYBOARD.lock().read();
        if code != self.last_value {
            let c = scancode::from_scancode(code as usize);
            match c {
                Some(x) => (self.callback)(x),
                None    => ()
            }
        }
        self.last_value = code;
    }
}

