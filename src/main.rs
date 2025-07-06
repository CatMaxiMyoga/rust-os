#![no_std]
#![no_main]

use core::panic::PanicInfo;
// use core::arch::asm;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo)-> ! { loop {} }

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again!").unwrap();
    write!(vga_buffer::WRITER.lock(), "  some numbers: {} {}", 42, 2.357)
        .unwrap();
    writeln!(vga_buffer::WRITER.lock(), "\nHello!").unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
