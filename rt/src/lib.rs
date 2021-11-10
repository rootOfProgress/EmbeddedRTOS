#![no_main]
#![no_std]
#![feature(asm)]
pub mod dev;

pub mod interrupts;
pub mod mem;
pub mod sched;
pub mod sys;
use core::panic::PanicInfo;
use interrupts::systick::{disable_systick, enable_systick};
use core::ptr;

use crate::sched::{scheduler, task_control};

fn enable_gpio_e_leds() {
    // see p 54 reg boundaries
    let gpio_port_e11 = dev::gpio_driver::GpioX::new("E", 11);
    gpio_port_e11.set_moder(dev::gpio_types::ModerTypes::GeneralPurposeOutputMode);
    gpio_port_e11.set_otyper(dev::gpio_types::OutputTypes::PushPull);
    let gpio_port_e14 = dev::gpio_driver::GpioX::new("E", 12);
    gpio_port_e14.set_moder(dev::gpio_types::ModerTypes::GeneralPurposeOutputMode);
    gpio_port_e14.set_otyper(dev::gpio_types::OutputTypes::PushPull);

    let gpio_port_e14 = dev::gpio_driver::GpioX::new("E", 14);
    gpio_port_e14.set_moder(dev::gpio_types::ModerTypes::GeneralPurposeOutputMode);
    gpio_port_e14.set_otyper(dev::gpio_types::OutputTypes::PushPull);

    let gpio_port_e14 = dev::gpio_driver::GpioX::new("E", 13);
    gpio_port_e14.set_moder(dev::gpio_types::ModerTypes::GeneralPurposeOutputMode);
    gpio_port_e14.set_otyper(dev::gpio_types::OutputTypes::PushPull);
}

fn setup_clock_system() {
    // turn on gpio clock
    // see p 166 -> IOPAEN
    let rcc_ahbenr = 0x40021000 | 0x14;
    unsafe { ptr::write_volatile(rcc_ahbenr as *mut u32, 1 << 17 | 1 << 21) }

    // TIM2EN -> p 166
    let rcc_apb1enr: u32 = 0x40021000 | 0x1C;
    unsafe {
        let existing_value = ptr::read_volatile(rcc_apb1enr as *mut u32);
        ptr::write_volatile(rcc_apb1enr as *mut u32, existing_value | 0b1);
    }

    // USART1EN -> p 166
    let rcc_apb2enr: u32 = 0x4002_1000 | 0x18;
    unsafe {
        let existing_value = ptr::read_volatile(rcc_apb2enr as *mut u32);
        ptr::write_volatile(rcc_apb2enr as *mut u32, existing_value | (0b1 << 14 | 0b1));
    }
}

fn enable_serial_printing() {
    let gpio_port_a0 = dev::gpio_driver::GpioX::new("A", 9);
    gpio_port_a0.set_moder(dev::gpio_types::ModerTypes::AlternateFunctionMode);
    gpio_port_a0.set_otyper(dev::gpio_types::OutputTypes::PushPull);
    gpio_port_a0.into_af(7);

    let usart1 = dev::uart::new(1, 9600);
    usart1.enable();
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    setup_clock_system();
    enable_gpio_e_leds();
    enable_serial_printing();
    // interrupts::systick::STK::set_up_systick(30);

    dev::uart::print_str("#########################\n\r");
    dev::uart::print_str("# WELCOME TO STM32 RTOS #\n\r");
    dev::uart::print_str("#########################\n\r");

    // dev::uart::print_dec(123);

    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

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

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

extern "C" {
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn PendSV();
    fn __set_exc_return();
}

#[no_mangle]
pub extern "C" fn SysTick() {
    unsafe {
        __set_exc_return();
    }
    sched::scheduler::context_switch();
}

#[no_mangle]
pub extern "C" fn SVCall() {
    unsafe {
        __set_exc_return();
        disable_systick();
        let sv_reason: u32;
        asm! ("mov {}, r2", out(reg) sv_reason);
        match sv_reason {
            0 => {
                asm!("bkpt")
            }
            1 => {
                scheduler::context_switch();
                // enable_systick();
                // asm!("bkpt")
            }
            2 => {
                task_control::terminate_task();
                scheduler::context_switch();
                // asm!("bkpt")
            }
            3 => {
                disable_systick();
                return;
            }
            4 => {
                enable_systick();
                return;
            }
            _ => {
                asm!("bkpt")
            }
        }
        enable_systick();
        
    }
}

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {}
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];
