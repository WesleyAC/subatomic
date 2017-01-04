extern crate spin;
extern crate cpuio;

use input::cpuio::Port;
use spin::Mutex;

pub static KEYBOARD: Mutex<Port<u8>> = Mutex::new(unsafe {
    Port::new(0x60)
});
