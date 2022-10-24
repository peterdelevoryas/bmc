#![no_std]
#![no_main]

use core::arch::global_asm;
use core::ptr;
use core::panic::PanicInfo;

trait Uart {
    const BASE_ADDR: u32;

    fn tx(&mut self, b: u8) {
        unsafe {
            ptr::write_volatile(Self::BASE_ADDR as *mut u32, b as u32);
        }
    }

    fn rx(&mut self) -> u8 {
        let r = unsafe {
            ptr::read_volatile(Self::BASE_ADDR as *const u32)
        };
        r as u8
    }
}

pub struct Uart5;

impl Uart for Uart5 {
    const BASE_ADDR: u32 = 0x1e78_4000;
}

global_asm!(include_str!("start.S"));

fn print(msg: &str) {
    for &b in msg.as_bytes() {
        Uart5.tx(b);
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
