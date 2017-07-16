pub mod color;
pub mod vga;
pub mod serial;

#[allow(dead_code)]
pub fn clear_screen() {
    vga::WRITER.lock().clear_screen()
}

#[allow(dead_code)]
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
        let mut vga_writer = console::vga::WRITER.lock();
        let mut serial_writer = console::serial::WRITER.lock();
        vga_writer.write_fmt(format_args!($($arg)*)).unwrap();
        serial_writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}
