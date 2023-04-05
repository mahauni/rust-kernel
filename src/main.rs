#![no_std] // dont link the rust to std library
#![no_main] // disable Rust-level entry points

use::core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle] // dont mangle the name of this function 
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since linker looks for a function
    // called '_start' by default
    println!("Hello World{}", "!");

    loop {}
}

// function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
