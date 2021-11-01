#![no_main]
#![no_std]
#![feature(asm)]
pub mod dev;

pub mod ctrl;
pub mod interrupts;
pub mod mem;
pub mod sched;

use core::panic::PanicInfo;
use core::ptr;
use core::mem::{MaybeUninit, zeroed};
use interrupts::systick;

use dev::tim2;

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

// propably the world's worst and slowest function to print stupid integers on
// a screen 
pub fn print_dec(mut dec: u32) {
    let usart2_tdr = 0x4001_3800 | 0x28;
    let usart2_isr = 0x4001_3800 | 0x1C;
    let mut buffer: [u8; 8] = unsafe { zeroed() };
    let mut cnt: u8 = 0;
    while dec > 0 {
        buffer[cnt as usize] = (dec % 10 + 0x30) as u8;
        dec /= 10;
        cnt += 1;
    }
    for c in buffer.into_iter().rev() {
        unsafe {
            ptr::write_volatile(usart2_tdr as *mut u32, *c as u32);
            while !((ptr::read_volatile(usart2_isr as *mut u32) & 0x80) != 0) {}
        }
    }
}

pub fn print_str(msg: &str) {
    let usart2_tdr = 0x4001_3800 | 0x28;
    let usart2_isr = 0x4001_3800 | 0x1C;

    for c in msg.chars() {
        unsafe {
            ptr::write_volatile(usart2_tdr as *mut u32, c as u32);
            while !((ptr::read_volatile(usart2_isr as *mut u32) & 0x80) != 0) {}
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    setup_clock_system();
    enable_gpio_e_leds();
    enable_serial_printing();
    interrupts::systick::STK::set_up_systick(200);
    sched::scheduler::init_task_mng();
    print_str("hello from rtos!...\n\r");

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
    fn __save_process_context();
    pub fn __exec(x: u32);
    // pub fn __exec_kernel(x: u32);
    pub fn __set_exec_mode(y: u32);
    pub fn __get_msp_entry() -> u32;
    pub fn __load_process_context();
    pub fn __set_exc_return();
}

#[no_mangle]
pub extern "C" fn SysTick() {
    // tim2::start_measurement();
    unsafe {
        __set_exc_return();
    }
    if sched::scheduler::usr_is_running() {
        unsafe {
            __save_process_context();
        }
        sched::task_control::update_tasks_ptr(ctrl::control::read_process_stack_ptr());
    }

    sched::scheduler::context_switch();
    if sched::scheduler::usr_is_running() {
        unsafe {
            __load_process_context();
        }
    }
    // tim2::stop_measurement();
    // let t = tim2::read_value();
    // print_str("context switch took: ");
    // print_dec(t);
    // print_str(" ns\n\r");
    // tim2::reset_timer();
}

#[no_mangle]
pub extern "C" fn SVCall() {
    sched::scheduler::run(0);
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
