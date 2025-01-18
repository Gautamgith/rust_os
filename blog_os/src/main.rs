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
    blog_os::hlt_loop();
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
    blog_os::init();
    
    fn stack_overflow() {
        stack_overflow();
    }
    // stack_overflow();

    // page fault
    //unsafe {
    //    *(0xdeadbeef as *mut u8) = 42;
    //};

    // Breakpoint here
    //x86_64::instructions::interrupts::int3();

    println!("I am speed!");
    #[cfg(test)]
    test_main();
    
    //loop {
    //    use blog_os::print;
    //    print!("-");        // new
    //}
    blog_os::hlt_loop();
}


#[test_case]
fn trivial_assertion() {
    // serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    // serial_println!("[ok]");
}

