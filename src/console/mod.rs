pub mod color;
pub mod vga;

use core::fmt::Write;

pub fn clear_screen() {
    vga::WRITER.lock().clear_screen()
}

pub fn set_color(color: color::ColorCode) {
    vga::WRITER.lock().set_color(color);
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = console::vga::WRITER.lock();
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}
