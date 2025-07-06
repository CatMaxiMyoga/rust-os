#![no_std]
#![no_main]

use core::panic::PanicInfo;
// use core::arch::asm;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo)-> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print!("\nHello!");
    println!("  Some numbers: {} {:.2}", 42, 2.357);
    panic!("Some Panic Message!!");

    /* Unreachable code */
    #[allow(clippy::empty_loop)]
    loop {}
}
