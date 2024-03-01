#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]


use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world{}", "!");
    panic!("THIS IS A PANIC");

    loop {}
}
