#![no_main]
#![no_std]

mod gpio;
use core::panic::PanicInfo;
use core::ptr;
fn foo() {
    // turn on gpio clock
    // see p 166 -> IOPAEN
    let rcc_ahbenr = 0x40021000 | 0x14;
    unsafe { ptr::write_volatile(rcc_ahbenr as *mut u32, 1 << 17) }

    // see p 54 reg boundaries
    // let gpioa_base = 0x48000000;
    let gpioa_base = gpio::gpio_driver::get_port("A");

    gpio::gpio_driver::set_moder(gpioa_base,gpio::gpio_driver::ModerTypes::GeneralPurposeOutputMode, 0);
    gpio::gpio_driver::set_otyper(gpioa_base,gpio::gpio_driver::OutputTypes::PushPull, 0);


    let gpioa_odr = gpioa_base | 0x14;
    unsafe {
        ptr::write_volatile(gpioa_odr as *mut u32, 1)
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