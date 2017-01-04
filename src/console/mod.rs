pub mod color;
pub mod vga;

use core::fmt::Write;

pub fn write_str(s: &str) {
    vga::WRITER.lock().write_str(s);
}

macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = console::vga::WRITER.lock();
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}
