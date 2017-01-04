extern crate spin;
extern crate cpuio;

use self::cpuio::Port;
use spin::Mutex;
use console;

static KEYBOARD: Mutex<Port<u8>> = Mutex::new(unsafe {
    Port::new(0x60)
});

pub struct PollingKeyboard {
    callback: fn(u8),
    last_value: u8
}

impl PollingKeyboard {
    pub fn new(callback: fn(u8)) -> PollingKeyboard {
        PollingKeyboard {
            callback: callback,
            last_value: 250 // Seems to be default value?
        }
    }

    pub fn update(&mut self) {
        let scancode = KEYBOARD.lock().read();
        if scancode != self.last_value {
            (self.callback)(scancode);
        }
        self.last_value = scancode;
    }
}

