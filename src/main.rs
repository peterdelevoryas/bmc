#![no_std]
#![no_main]
#![feature(ptr_as_uninit)]

use core::arch::asm;
use core::arch::global_asm;
use core::ptr;
use core::panic::PanicInfo;
use core::ops::RangeInclusive;

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

type Bitfield = RangeInclusive<u8>;

const TRANSMITTER_EMPTY: Bitfield = 6..=6;
const ENABLE_UART_FIFO: Bitfield = 0..=0;

fn bitmask(field: Bitfield) -> u32 {
    (1 << (field.end() - field.start() + 1)) - 1
}

struct Reg32 {
    value: u32,
}

impl Reg32 {
    fn get(&self) -> u32 {
        unsafe { ptr::read_volatile(&self.value) }
    }

    fn set(&mut self, value: u32) {
        unsafe { ptr::write_volatile(&mut self.value, value) }
    }

    fn field(&self, field: Bitfield) -> u32 {
        let mut v = self.get();
        v >>= field.start();
        v &= bitmask(field);
        v
    }

    fn set_field(&mut self, field: Bitfield, value: u32) {
        let mut v = self.get();
        let mask = bitmask(field.clone());
        v &= !(mask << field.start());
        v |= (value & mask) << field.start();
        self.set(v);
    }
}

#[repr(C)]
struct Uart {
    rbr_thr: Reg32,
    ier: Reg32,
    fcr: Reg32,
    lcr: Reg32,
    mcr: Reg32,
    lsr: Reg32,
}

impl Uart {
    fn tx_empty(&self) -> bool {
        self.lsr.field(TRANSMITTER_EMPTY) == 1
    }

    fn print(&mut self, msg: &str) {
        for &b in msg.as_bytes() {
            self.push(b);
        }
    }

    fn push(&mut self, b: u8) {
        while !self.tx_empty() {
        }
        self.rbr_thr.set(b as u32);
    }
}

#[no_mangle]
pub extern "C" fn main(cpuid: i32) -> ! {
    let uart5 = UART5 as *mut Uart;
    let uart5 = unsafe { &mut *uart5 };

    uart5.fcr.set_field(ENABLE_UART_FIFO, 1);
    uart5.print("hello world\r\n");

    loop {
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

global_asm!(include_str!("start.S"));
