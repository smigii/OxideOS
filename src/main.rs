// Since we do not have access to crt0,
// or the Rust libraries.
#![no_main]
#![no_std]

// Notice that
// there is no main(), since there is no
// underlying runtime to call it.

use core::panic::PanicInfo;

// This will be called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

// Guarantees that the Rust compiler gives us a function
// named _start. extern "C" tells compiler that it should
// use C Calling Convention.
// https://en.wikipedia.org/wiki/Calling_convention
// https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}