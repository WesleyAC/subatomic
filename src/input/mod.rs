mod x86;
mod cpuio;

extern crate spin;
use spin::Mutex;
use input::cpuio::Port;

/// Port used to access a PS/2 keyboard.
pub static KEYBOARD: Mutex<Port<u8>> = Mutex::new(unsafe {
    Port::new(0x60)
});
