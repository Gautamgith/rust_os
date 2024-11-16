#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;


// This func is called onPanic by compiler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

// static HELLO: &[u8] = b"Red rust is a mood!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    println!("Red Rust is {}!!", "eternal");
    
    #[cfg(test)]
    test_main();

    loop {}
}


#[test_case]
fn trivial_assertion() {
    // serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    // serial_println!("[ok]");
}

