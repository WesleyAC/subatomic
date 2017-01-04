pub mod color;
pub mod vga;


pub fn write_str(s: &str) {
    vga::WRITER.lock().write_str(s);
}
