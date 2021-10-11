#![no_main]
#![no_std]

mod gpio;
use core::panic::PanicInfo;
use core::ptr;
fn foo() {
    // turn on gpio clock
    // see p 166 -> IOPAEN
    let rcc_ahbenr = 0x40021000 | 0x14;
    unsafe { ptr::write_volatile(rcc_ahbenr as *mut u32, (1 << 17 | 1 << 21)) }

    // see p 54 reg boundaries
    let gpioa_base = gpio::gpio_driver::get_port("A");
    gpio::gpio_driver::set_moder(gpioa_base,gpio::gpio_types::ModerTypes::GeneralPurposeOutputMode, 0);
    gpio::gpio_driver::set_otyper(gpioa_base,gpio::gpio_types::OutputTypes::PushPull, 0);
    gpio::gpio_driver::set_odr(gpioa_base,gpio::gpio_types::OutputState::High, 0);

    // roulette leds -> STILL BUGGY DOES NOT WORK PROPERLY!!!
    let gpioe_base = gpio::gpio_driver::get_port("E");
    gpio::gpio_driver::set_moder(gpioe_base,gpio::gpio_types::ModerTypes::GeneralPurposeOutputMode, 9);
    gpio::gpio_driver::set_otyper(gpioe_base,gpio::gpio_types::OutputTypes::PushPull, 9);
    gpio::gpio_driver::set_odr(gpioe_base,gpio::gpio_types::OutputState::High, 9);

    gpio::gpio_driver::set_moder(gpioe_base,gpio::gpio_types::ModerTypes::GeneralPurposeOutputMode, 11);
    gpio::gpio_driver::set_otyper(gpioe_base,gpio::gpio_types::OutputTypes::PushPull, 11);
    gpio::gpio_driver::set_odr(gpioe_base,gpio::gpio_types::OutputState::High, 11);

    gpio::gpio_driver::set_moder(gpioe_base,gpio::gpio_types::ModerTypes::GeneralPurposeOutputMode, 14);
    gpio::gpio_driver::set_otyper(gpioe_base,gpio::gpio_types::OutputTypes::PushPull, 14);
    gpio::gpio_driver::set_odr(gpioe_base,gpio::gpio_types::OutputState::High, 14);
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