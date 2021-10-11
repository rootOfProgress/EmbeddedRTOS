#![no_std]
#![no_main]

use core::ptr;

extern crate rt;

static RODATA: &[u8] = b"Hello, world!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;

const RCC_AHBENR: u32 = 0x4002_1000 | 0x14;

const GPIOE_BASE: u32 = 0x4800_1000;

#[repr(C)]
struct RCC {
    pub ahbenr: u32,
}

#[repr(C)]
struct GPIO {
    pub moder: u32,
    pub otyper: u32,
    pub ospeedr: u32,
    pub pupdr: u32,
    pub idr: u32,
    pub odr: u32,
    pub bsrr: u32,
    pub lckr: u32,
    pub afrl: u32,
    pub afrh: u32,
    pub brr: u32,
}

#[no_mangle]
fn main() -> ! {

    let rcc = unsafe { &mut *(RCC_AHBENR as *mut RCC) };

    // IOPAEN p.166 "io port a enable"
    unsafe {ptr::write_volatile(&mut rcc.ahbenr as *mut u32, ptr::read_volatile(&mut rcc.ahbenr) | (1 << 17 | 1 << 21))};
    
    let gpioe = unsafe { &mut *(GPIOE_BASE as *mut GPIO) };
    
    let pin = 9;
    unsafe {
        ptr::write_volatile(&mut gpioe.moder as *mut u32, ptr::read_volatile(&mut gpioe.moder) | (0b01 as u32) << (pin * 2));
        ptr::write_volatile(&mut gpioe.otyper as *mut u32, ptr::read_volatile(&mut gpioe.otyper) & !(1 as u32) << pin);
        ptr::write_volatile(&mut gpioe.odr as *mut u32, ptr::read_volatile(&mut gpioe.odr) | (0b1 as u32) << pin);
    }

    let _x = RODATA;
    let _y = unsafe { &BSS };
    let _z = unsafe { &DATA };

    loop {}
}
