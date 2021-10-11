#![no_std]
#![no_main]

use core::ptr;

extern crate rt;

static RODATA: &[u8] = b"Hello, world!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;

const RCC_AHBENR: u32 = 0x4002_1000 | 0x14;

const GPIOE_BASE: u32 = 0x4800_0000;

#[repr(C)]
struct RCC {
    pub ahbenr: u32,
}

#[repr(C)]
struct GPIO {
    moder: u32,
    otyper: u32,
    ospeedr: u32,
    pupdr: u32,
    idr: u32,
    odr: u32,
    bsrr: u32,
    lckr: u32,
    afrl: u32,
    afrh: u32,
    brr: u32,
}

#[no_mangle]
fn main() -> ! {

    let rcc = unsafe { &mut *(RCC_AHBENR as *mut RCC) };

    // IOPAEN p.166 "io port a enable"
    unsafe {ptr::write_volatile(&mut (*rcc).ahbenr as *mut u32, ptr::read_volatile(&mut rcc.ahbenr) | 1 << 17)};
    
    let gpioe = unsafe { &mut *(GPIOE_BASE as *mut GPIO) };
    
    unsafe {
        ptr::write_volatile(&mut gpioe.moder as *mut u32, ptr::read_volatile(&mut gpioe.moder) | (0b01 as u32) << (9 * 2));
        ptr::write_volatile(&mut gpioe.otyper as *mut u32, ptr::read_volatile(&mut gpioe.otyper) & !(1 as u32) << 9);
        ptr::write_volatile(&mut gpioe.odr as *mut u32, ptr::read_volatile(&mut gpioe.odr) | 1 << 9);
    }

    let _mod = unsafe {ptr::read_volatile(&mut (*gpioe).moder)};

    let _x = RODATA;
    let _y = unsafe { &BSS };
    let _z = unsafe { &DATA };

    loop {}
}
