#![no_std]

use core::panic::PanicInfo;

/// This func is called onPanic by compiler
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

fn main() {
}
