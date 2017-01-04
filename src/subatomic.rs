#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;

pub mod panic;
mod vga;

#[no_mangle]
pub extern fn kmain() -> ! {
    vga::WRITER.lock().write_str("Welcome to subatomic");
    loop{}
}

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
