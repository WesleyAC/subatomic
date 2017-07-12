// This file defines what happens on a panic, as well as implementing a few
// other functions that are needed in order to compile.
use core::fmt;
use console;

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    console::set_color(console::color::ColorCode::new(console::color::Color::Red, console::color::Color::Black));
    println!("PANIC!");
    println!("{}:{}", _file, _line);
    println!("{}", _msg);
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
