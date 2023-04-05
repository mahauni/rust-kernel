#![no_std] // dont link the rust to std library
#![no_main] // disable Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use ::core::panic::PanicInfo;
use rust_kernel::println;

#[no_mangle] // dont mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since linker looks for a function
    // called '_start' by default
    println!("Hello World{}", "!");

    rust_kernel::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash");
    rust_kernel::hlt_loop();
}

// function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    
    rust_kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_kernel::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
