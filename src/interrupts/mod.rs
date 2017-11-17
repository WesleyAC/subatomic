use x86_64::structures::idt::Idt;
use x86_64::structures::idt::ExceptionStackFrame;
#[macro_use]
use console;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame)
{
    console::set_color(console::color::ColorCode::new(console::color::Color::Red, console::color::Color::Black));
    println!("Exception caught: Breakpoint\n{:#?}", stack_frame);
}
