// Indicate to the compiler that the standard library should not be linked to this executable.
// The standard library depends on having an operating system available, however this executable is
// intend to be run as an operating system, so it doesn't have an operating system available.
#![no_std]
// In a standard binary, there is a little runtime setup compiled into the final binary by the
// language compiler, then it links to the `main` function. This attribute tells the Rust compiler
// to skip that stuff for this binary. See the `_start` function for how this is all replaced.
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// This function is called if there is a panic. Normally, the standard libary would supply a panic
// function, but we are linking the standard library, so we have to do this manully.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Instead of using the default compiler offering (runtime setup before invoking `main`), we will
// designate our own function for the linker to start with. Don't mangle this function name because
// we need to to be exported from the compiled binary object just as is. The linker will look for a
// function named `_start` by default. The `-> !` syntax indicates this function will never return.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
