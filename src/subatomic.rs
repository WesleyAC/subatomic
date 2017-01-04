#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;

pub mod panic;
mod console;

#[no_mangle]
pub extern fn kmain() -> ! {
    console::write_str("Welcome to subatomic");
    loop{}
}
