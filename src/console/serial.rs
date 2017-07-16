extern crate cpuio;

use spin::Mutex;
use self::cpuio::Port;
use core::fmt;

const COM1: u16 = 0x3F8;
const COM2: u16 = 0x2F8;
const COM3: u16 = 0x3E8;
const COM4: u16 = 0x2E8;

pub struct SerialConsole {
    data: Port<u8>,
    int_enable: Port<u8>,
    int_id: Port<u8>,
    line_control: Port<u8>,
    modem_control: Port<u8>,
    line_status: Port<u8>,
    modem_status: Port<u8>,
    scratch: Port<u8>,
    initalized: bool
}

impl SerialConsole {
    pub const unsafe fn new(port: u16) -> Mutex<Self> {
        Mutex::new(SerialConsole {
            data:          unsafe { Port::new(port) },
            int_enable:    unsafe { Port::new(port + 1) },
            int_id:        unsafe { Port::new(port + 2) },
            line_control:  unsafe { Port::new(port + 3) },
            modem_control: unsafe { Port::new(port + 4) },
            line_status:   unsafe { Port::new(port + 5) },
            modem_status:  unsafe { Port::new(port + 6) },
            scratch:       unsafe { Port::new(port + 7) },
            initalized:    false
        })
    }

    //TODO(Wesley) allow setting baud rate, pairity, etc
    pub fn init(&mut self) {
        self.int_enable.write(0x00);    // Disable interrupts
        self.line_control.write(0x80);  // Enable DLAB
        self.data.write(0x03);          // Set divisor to 3 (lo byte) 38400 baud
        self.int_enable.write(0x00);    //                  (hi byte)
        self.line_control.write(0x03);  // 8 bits, no parity, one stop bit
        self.int_id.write(0xC7);        // Enable FIFO, clear them, with 14-byte threshold
        self.modem_control.write(0x03); // IRQs disabled, RTS/DSR set
    }

    #[inline]
    fn init_if_needed(&mut self) {
        if !self.initalized {
            self.init();
        }
    }

    #[inline]
    pub fn has_data(&mut self) -> bool {
        self.init_if_needed();
        (self.line_status.read() & 1) != 0
    }

    pub fn write(&mut self, s: &str) {
        self.init_if_needed();
        for c in s.bytes() {
            self.data.write(c);
        }
    }

    pub fn read(&mut self) -> u8 {
        self.init_if_needed();
        self.data.read()
    }
}

impl fmt::Write for SerialConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}

pub static WRITER: Mutex<SerialConsole> = unsafe { SerialConsole::new(COM1) };
