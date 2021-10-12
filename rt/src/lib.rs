#![no_main]
#![no_std]

mod gpio;
use gpio::{gpio_driver, gpio_types};

use core::panic::PanicInfo;
use core::ptr;
fn foo() {
    // turn on gpio clock
    // see p 166 -> IOPAEN
    let rcc_ahbenr = 0x40021000 | 0x14;
    unsafe { ptr::write_volatile(rcc_ahbenr as *mut u32, (1 << 17 | 1 << 21)) }

    // see p 54 reg boundaries

    let gpio_port_a0 = gpio_driver::GpioX::new("A", 0);
    gpio_port_a0.set_moder(gpio_types::ModerTypes::GeneralPurposeOutputMode);
    gpio_port_a0.set_otyper(gpio_types::OutputTypes::PushPull);
    gpio_port_a0.set_odr(gpio_types::OutputState::High);

    let gpio_port_e9 = gpio::gpio_driver::GpioX::new("E", 9);
    gpio_port_e9.set_moder(gpio::gpio_types::ModerTypes::GeneralPurposeOutputMode);
    gpio_port_e9.set_otyper(gpio::gpio_types::OutputTypes::PushPull);
    gpio_port_e9.set_odr(gpio::gpio_types::OutputState::High);

    let gpio_port_e11 = gpio::gpio_driver::GpioX::new("E", 11);
    gpio_port_e11.set_moder(gpio::gpio_types::ModerTypes::GeneralPurposeOutputMode);
    gpio_port_e11.set_otyper(gpio::gpio_types::OutputTypes::PushPull);
    gpio_port_e11.set_odr(gpio::gpio_types::OutputState::High);

    let gpio_port_e14 = gpio::gpio_driver::GpioX::new("E", 14);
    gpio_port_e14.set_moder(gpio::gpio_types::ModerTypes::GeneralPurposeOutputMode);
    gpio_port_e14.set_otyper(gpio::gpio_types::OutputTypes::PushPull);
    gpio_port_e14.set_odr(gpio::gpio_types::OutputState::High);
    // let gpioe_base = gpio::gpio_driver::get_port("E");
    // gpio::gpio_driver::set_moder(gpioe_base,gpio::gpio_types::ModerTypes::GeneralPurposeOutputMode, 9);
    // gpio::gpio_driver::set_otyper(gpioe_base,gpio::gpio_types::OutputTypes::PushPull, 9);
    // gpio::gpio_driver::set_odr(gpioe_base,gpio::gpio_types::OutputState::High, 9);

    // gpio::gpio_driver::set_moder(gpioe_base,gpio::gpio_types::ModerTypes::GeneralPurposeOutputMode, 11);
    // gpio::gpio_driver::set_otyper(gpioe_base,gpio::gpio_types::OutputTypes::PushPull, 11);
    // gpio::gpio_driver::set_odr(gpioe_base,gpio::gpio_types::OutputState::High, 11);

    // gpio::gpio_driver::set_moder(gpioe_base,gpio::gpio_types::ModerTypes::GeneralPurposeOutputMode, 14);
    // gpio::gpio_driver::set_otyper(gpioe_base,gpio::gpio_types::OutputTypes::PushPull, 14);
    // gpio::gpio_driver::set_odr(gpioe_base,gpio::gpio_types::OutputState::High, 14);

}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    foo();
    extern "Rust" {
        fn main() -> !;
    }

    main()
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}