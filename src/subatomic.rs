#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(asm)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod console;
mod input;
mod memory;
mod panic;

use memory::FrameAllocator;

#[no_mangle]
pub extern fn kmain(multiboot_info_addr: usize) -> ! {
    console::set_color(console::color::ColorCode::new(console::color::Color::LightBlue, console::color::Color::Black));
    println!("Welcome to subatomic");
    console::set_color(console::color::ColorCode::new(console::color::Color::Green, console::color::Color::Black));

    let mut frame_allocator = setup_allocator(multiboot_info_addr);
    let mut kbd: input::poll::PollingKeyboard = input::poll::PollingKeyboard::new(handle_char);

    loop{
        kbd.update();
    }
}

fn setup_allocator(multiboot_info_addr: usize) -> memory::AreaFrameAllocator {
    let boot_info = unsafe{ multiboot2::load(multiboot_info_addr) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    let multiboot_start = multiboot_info_addr;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    memory::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas())
}

fn handle_char(c: char) {
    print!("{}", c);
}
