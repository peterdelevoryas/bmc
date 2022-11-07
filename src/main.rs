#![no_std]
#![no_main]
#![feature(ptr_as_uninit)]

use core::arch::asm;
use core::arch::global_asm;
use core::ptr;
use core::panic::PanicInfo;

const UART1: u32 = 0x1E78_3000;
const UART2: u32 = 0x1E78_D000;
const UART3: u32 = 0x1E78_E000;
const UART4: u32 = 0x1E78_F000;
const UART5: u32 = 0x1E78_4000;
const UART6: u32 = 0x1E79_0000;
const UART7: u32 = 0x1E79_0100;
const UART8: u32 = 0x1E79_0200;
const UART9: u32 = 0x1E79_0300;
const UART10: u32 = 0x1E79_0400;
const UART11: u32 = 0x1E79_0500;
const UART12: u32 = 0x1E79_0600;
const UART13: u32 = 0x1E79_0700;

const UART_RBR: u32 = 0x00;
const UART_THR: u32 = 0x00;
const UART_IER: u32 = 0x04;
const UART_IIR: u32 = 0x08;
const UART_FCR: u32 = 0x08;
const UART_LCR: u32 = 0x0C;
const UART_MCR: u32 = 0x10;
const UART_LSR: u32 = 0x14;
const UART_MSR: u32 = 0x18;
const UART_SCR: u32 = 0x1C;

#[repr(C)]
struct Uart {
    rbr_thr: u32,
    ier: u32,
    fcr: u32,
    lcr: u32,
    mcr: u32,
    lsr: u32,
}

impl Uart {
    fn print(&mut self, msg: &str) {
        self.write(msg.as_bytes());
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            while self.lsr & 0b01000000 == 0 {
            }
            self.rbr_thr = b as u32;
        }
    }
}

#[no_mangle]
pub extern "C" fn main(cpuid: i32) -> ! {
    let uart5 = UART5 as *mut Uart;
    let uart5 = unsafe { &mut *uart5 };

    uart5.fcr = 1;
    uart5.write(&[b'0' + cpuid as u8]);

    loop {
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

global_asm!(include_str!("start.S"));
