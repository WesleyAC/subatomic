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
