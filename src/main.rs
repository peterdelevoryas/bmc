#![no_std]
#![no_main]
#![feature(ptr_as_uninit, const_mut_refs)]
#![allow(warnings)]

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

fn bitmask(field: Bitfield) -> u32 {
    (1 << (field.end() - field.start() + 1)) - 1
}

#[repr(C)]
struct Reg {
    value: u32,
}

impl Reg {
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


struct RegField<const START: u8, const END: u8>;

impl<const START: u8, const END: u8> RegField<START, END> {
    fn get(&self) -> u32 {
        let ptr = self as *const Self as *const u32;
        let mut reg = unsafe { ptr::read_volatile(ptr) };
        reg >>= START;
        reg &= bitmask(START..=END);
        reg
    }

    fn set(&mut self, v: u32) {
        let ptr = self as *mut Self as *mut u32;
        let mut reg = unsafe { ptr::read_volatile(ptr) };
        let mask = bitmask(START..=END);
        reg &= !(mask << START);
        reg |= (v & mask) << START;
        unsafe { ptr::write_volatile(ptr, reg); }
    }
}

#[repr(C)]
struct Uart {
    rbr_thr: RbrThr,
    ier: Ier,
    fcr: Fcr,
    lcr: Reg,
    mcr: Reg,
    lsr: Lsr,
}

impl Uart {
    fn print_reg(&mut self, reg: u32) {
        self.print("0x");
        for i in (0..8).rev() {
            let shift = i * 4;
            let b = (reg >> shift) as u8;
            let b = match b {
                0..=9 => b'0' + b,
                _ => b'a' + b,
            };
            self.push(b);
        }
    }

    fn print(&mut self, msg: &str) {
        for &b in msg.as_bytes() {
            self.push(b);
        }
    }

    fn push(&mut self, b: u8) {
        while self.lsr.transmitter_empty.get() == 0 {
        }
        self.rbr_thr.thr.set(b as u32);
    }
}

#[repr(C)]
struct RbrThr {
    rbr: RegField<0, 8>,
    thr: RegField<0, 8>,
    reg: Reg,
}

#[repr(C)]
struct Fcr {
    enable_uart_fifo: RegField<0, 0>,
    reg: Reg,
}

#[repr(C)]
struct Ier {
    enable_received_data_available_interrupt: RegField<0, 0>,
    reg: Reg,
}

#[repr(C)]
struct Lsr {
    transmitter_empty: RegField<6, 6>,
    reg: Reg,
}

#[no_mangle]
pub extern "C" fn main(cpuid: i32) -> ! {
    let uart5 = unsafe { &mut *(UART5 as *mut Uart) };

    uart5.fcr.enable_uart_fifo.set(1);
    uart5.print("hello world\n");

    uart5.ier.enable_received_data_available_interrupt.set(1);
    uart5.print_reg(uart5.ier.reg.get());
    uart5.push(b'\n');
    loop {
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

global_asm!(include_str!("start.S"));
