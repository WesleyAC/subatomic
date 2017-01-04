#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;


#[macro_use]
mod console;
pub mod panic;

#[no_mangle]
pub extern fn kmain(multiboot_information_address: usize) -> ! {
    console::set_color(console::color::ColorCode::new(console::color::Color::LightBlue, console::color::Color::Black));
    println!("Welcome to subatomic");
    console::set_color(console::color::ColorCode::new(console::color::Color::Green, console::color::Color::Black));
    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
            area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    println!("Kernel:");
    println!("Start: 0x{:x} End: 0x{:x}", kernel_start, kernel_end);

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("Multiboot:");
    println!("Start: 0x{:x} End: 0x{:x}", multiboot_start, multiboot_end);

    loop{}
}
