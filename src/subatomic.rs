#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

pub mod panic;
mod console;

#[no_mangle]
pub extern fn kmain(multiboot_information_address: usize) -> ! {
    console::write_str("Welcome to subatomic");
    loop{}
}
