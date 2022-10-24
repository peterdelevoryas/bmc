#![no_std]
#![no_main]

use core::arch::global_asm;
use core::ptr;
use core::panic::PanicInfo;

mod ast2600 {
    pub const UART5: u32 = 0x1E78_4000;
}

global_asm!(include_str!("start.S"));

fn print(msg: &str) {
    for &b in msg.as_bytes() {
        unsafe {
            ptr::write(ast2600::UART5 as *mut u32, b as u32);
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    print("hello world\n");
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
