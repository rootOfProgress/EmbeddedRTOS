#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::ptr;
fn foo() {
    // turn on gpio clock
    // see p 166 -> IOPAEN
    let rcc_ahbenr = 0x40021000 | 0x14;
    unsafe { ptr::write_volatile(rcc_ahbenr as *mut u32, 1 << 17) }

    // see p 54 reg boundaries
    let gpioa_base = 0x48000000;
    let gpioa_moder = gpioa_base | 0x00;
    unsafe {
        let mut existing_val = ptr::read_volatile(gpioa_moder as *const u32);

        // clear out first bit (LSB)
        existing_val &= !(1 << 0);

        // clear out second bit (LSB)
        existing_val &= !(1 << 1);

        // set bit 0 & 1 to 0b01 (see p. 237, "01: General purpose output mode") first bit (LSB)
        existing_val |= (0b01) | (0b00);

        ptr::write_volatile(gpioa_moder as *mut u32,existing_val);
    }
    let gpioa_otyper = gpioa_base | 0x04;
    unsafe {
        // see p 237
        // 0: Output push-pull (reset state)
        ptr::write_volatile(gpioa_otyper as *mut u32, 0)
    }

    let gpioa_odr = gpioa_base | 0x14;
    unsafe {
        // see p 239
        // 0: Output push-pull (reset state)
        ptr::write_volatile(gpioa_odr as *mut u32, 1)
    }

    // unsafe {
    //     // see p 239
    //     // 0: Output push-pull (reset state)
    //     ptr::write_volatile(gpioa_odr as *mut u32, 0)
    // }
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    let _x = 66;
    foo();
    loop {}
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
