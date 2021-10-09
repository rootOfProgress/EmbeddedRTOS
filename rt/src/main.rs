#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::ptr;
fn foo() {
    let rcc_ahbenr = 0x40021000 | 0x14;
    unsafe {
        ptr::write_volatile(rcc_ahbenr as *mut u32, 1 << 17)
    }

    let gpioa_base = 0x48000000;
    let gpioa_moder = gpioa_base | 0x00;
    unsafe {
        let mut existing_val = ptr::read_volatile(gpioa_moder as *const u32);
        existing_val = existing_val & !(0b11) & !(0b11); 

        ptr::write_volatile(
            gpioa_moder as *mut u32, 
            existing_val | (0b01) | (0b00)
        );
    }
    // unsafe {
    //     // see p 237
    //     // 01: General purpose output mode
    //     ptr::write_volatile(gpioa_base as *mut u32, 1)
    // }

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

    unsafe {
        // see p 239
        // 0: Output push-pull (reset state)
        ptr::write_volatile(gpioa_odr as *mut u32, 0)
    }
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
