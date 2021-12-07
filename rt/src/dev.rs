use super::mem;
use super::platform;

pub mod tim3 {
    use super::mem::memory_handler::{read, write};
    use super::platform::{adresses, offsets};
    const TIM3_BASE: u32 = adresses::TIM3_BASEADRESS;
    use core::ptr;

    pub fn start() {
        write(TIM3_BASE, read(TIM3_BASE) | 0b1);
    }

    pub fn clear_udis() {
        write(TIM3_BASE, read(TIM3_BASE) & !(0b10));
    }

    pub fn flush() {
        let timx_cnt: u32 = TIM3_BASE | offsets::tim::CNT;
        write(timx_cnt, read(timx_cnt) & !(0xFFFF));
    }

    pub fn clear_uif() {
        let tim3_sr = TIM3_BASE | 0x10;
        unsafe {
            let existing_value = read(tim3_sr);
            ptr::write_volatile(tim3_sr as *mut u32, existing_value & !(0b1111));
            ptr::write_volatile(tim3_sr as *mut u32, existing_value & !(0b1 << 6));
        }
    }

    pub fn set_ug() {
        let tim3_egr = TIM3_BASE | offsets::tim::EGR;
        unsafe {
            let existing_value = read(tim3_egr);
            ptr::write_volatile(tim3_egr as *mut u32, existing_value | 0b1);
        }
    }

    pub fn stop() {
        write(TIM3_BASE, read(TIM3_BASE) & !(0b1));
    }

    pub fn enable_interrupt() {
        let tim3_dier: u32 = TIM3_BASE | offsets::tim::DIER;
        write(tim3_dier, read(tim3_dier) | 0b10);
    }

    pub fn set_ccr(threshold: u16) {
        let tim3_ccr1: u32 = TIM3_BASE | offsets::tim::CCR1;
        write(tim3_ccr1, read(tim3_ccr1) | threshold as u32);
    }

    pub fn set_prescaler(value: u16) {
        let tim3_psc: u32 = TIM3_BASE | offsets::tim::PSC;
        write(tim3_psc, read(tim3_psc) | value as u32);
    }
    pub fn reset_timer() {
        // TIM3 RESET -> p 166
        let rcc_apb1rstr: u32 = 0x4002_1000 | 0x10;
        unsafe {
            let existing_value = ptr::read_volatile(rcc_apb1rstr as *mut u32);
            ptr::write_volatile(rcc_apb1rstr as *mut u32, existing_value | 0b10);
            let existing_value = ptr::read_volatile(rcc_apb1rstr as *mut u32);
            ptr::write_volatile(rcc_apb1rstr as *mut u32, existing_value & !(0b10));
        }
    }
    pub fn read_value() -> u32 {
        let timx_cnt: u32 = TIM3_BASE | 0x24;
        unsafe { (ptr::read_volatile(timx_cnt as *mut u32) & !(0b1 << 31)) * 125 }
    }
}

pub mod tim2 {
    use super::mem::memory_handler::{read, write};
    use super::platform::{adresses, offsets};

    const TIM2_CR1: u32 = adresses::TIM2_BASEADRESS;
    use core::ptr;

    pub fn start_measurement() {
        write(TIM2_CR1, read(TIM2_CR1) | 0b1);
    }
    pub fn stop_measurement() {
        write(TIM2_CR1, read(TIM2_CR1) & !(0b1));
    }
    pub fn reset_timer() {
        // TIM2 RESET -> p 166
        let rcc_apb1rstr: u32 = adresses::RCC | offsets::rcc::RCC_APB1RSTR;
        unsafe {
            let existing_value = ptr::read_volatile(rcc_apb1rstr as *mut u32);
            ptr::write_volatile(rcc_apb1rstr as *mut u32, existing_value | 0b1);
            let existing_value = ptr::read_volatile(rcc_apb1rstr as *mut u32);
            ptr::write_volatile(rcc_apb1rstr as *mut u32, existing_value & !(0b1));
        }
    }
    pub fn read_value() -> u32 {
        let timx_cnt: u32 = adresses::TIM2_BASEADRESS | offsets::tim::CNT;
        (read(timx_cnt) & !(0b1 << 31)) * 125
    }
}

// NOTE: ONLY FOR TESTPURPOSES, NO GENERIC USART DRIVER YET!
pub mod uart {
    use super::mem::memory_handler::{read, write};
    const USART1_BASE: u32 = adresses::USART1_BASEADRESS;
    use core::ptr;

    use crate::generic::platform::{adresses, offsets};
    pub struct UsartX {
        usart_base_adress: u32,
        bus_number: u8,
        baudrate: u32,
    }
    pub fn new(bus_number: u8, baudrate: u32) -> UsartX {
        UsartX {
            usart_base_adress: match bus_number {
                1 => USART1_BASE,
                _ => USART1_BASE,
            },
            bus_number,
            baudrate,
        }
    }
    impl UsartX {
        pub fn enable(&self) {
            let usartx_brr = self.usart_base_adress | 0x0C;
            let baudrate_divisor = 8_000_000 / self.baudrate;
            // clk / 9600 baud
            write(usartx_brr, baudrate_divisor);

            let usart1_cr1 = self.usart_base_adress;
            let existing_value = read(usart1_cr1);
            write(usart1_cr1, existing_value | (0b1100));
            let existing_value = read(usart1_cr1);
            write(usart1_cr1, existing_value | (0b1));
        }
    }

    // propably the world's worst and slowest function to print stupid integers on
    // a screen
    pub fn print_dec(mut dec: u32) {
        let usart2_tdr = adresses::USART1_BASEADRESS | offsets::usart1::TDR;
        let usart2_isr = adresses::USART1_BASEADRESS | offsets::usart1::ISR;
        let mut buffer: [u8; 32] = unsafe { core::mem::zeroed() };
        let mut cnt: u8 = 0;
        while dec > 0 {
            buffer[cnt as usize] = (dec % 10 + 0x30) as u8;
            dec /= 10;
            cnt += 1;
        }
        for c in IntoIterator::into_iter(buffer).rev() {
            unsafe {
                ptr::write_volatile(usart2_tdr as *mut u32, c as u32);
                while !((ptr::read_volatile(usart2_isr as *mut u32) & 0x80) != 0) {}
            }
        }
    }

    pub fn print_from_ptr(mut ptr_start: *mut u8) {
        let usart2_tdr = adresses::USART1_BASEADRESS | offsets::usart1::TDR;
        let usart2_isr = adresses::USART1_BASEADRESS | offsets::usart1::ISR;

        unsafe {
            while *ptr_start != b'\0' {
                write(usart2_tdr, *ptr_start as u32);
                ptr_start = ptr_start.add(1);
                while !((read(usart2_isr) & 0x80) != 0) {}
            }
        }
    }

    pub fn print_str(msg: &str) {
        let usart2_tdr = adresses::USART1_BASEADRESS | offsets::usart1::TDR;
        let usart2_isr = adresses::USART1_BASEADRESS | offsets::usart1::ISR;

        for c in msg.chars() {
            write(usart2_tdr, c as u32);
            while !((read(usart2_isr) & 0x80) != 0) {}
        }
    }
}

pub mod gpio_types {

    pub enum OutputTypes {
        PushPull,
        OpenDrain,
    }

    pub enum ModerTypes {
        InputMode,
        GeneralPurposeOutputMode,
        AlternateFunctionMode,
        AnalogMode,
    }

    pub enum OutputState {
        High,
        Low,
    }
}

pub mod gpio_driver {
    use super::gpio_types;
    use core::ptr;

    // p52
    const GPIOA_BASE: u32 = 0x4800_0000;
    const GPIOB_BASE: u32 = 0x4800_0400;
    const GPIOC_BASE: u32 = 0x4800_0800;
    const GPIOE_BASE: u32 = 0x4800_1000;

    pub struct GpioX {
        gpio_base_adress: u32,
        pin_number: u8,
    }

    impl GpioX {
        pub fn new(port_mnemonic: &str, pin_number: u8) -> GpioX {
            GpioX {
                gpio_base_adress: match port_mnemonic {
                    "A" => GPIOA_BASE,
                    "B" => GPIOB_BASE,
                    "C" => GPIOC_BASE,
                    "E" => GPIOE_BASE,
                    _ => GPIOA_BASE,
                },
                pin_number,
            }
        }
        pub fn set_moder(&self, moder_type: gpio_types::ModerTypes) {
            let gpiox_moder_offset = 0x00;
            let gpiox_moder = self.gpio_base_adress | gpiox_moder_offset;

            // 32 bit register
            let mut current_register_content: u32;

            unsafe {
                current_register_content = ptr::read_volatile(gpiox_moder as *const u32);
            }
            // clear bits
            // current_register_content &= !(0b11 as u32) << pin * 2;

            let updated_register_content = match moder_type {
                // clear bit
                gpio_types::ModerTypes::InputMode => {
                    current_register_content |= (0b00 as u32) << (self.pin_number * 2);
                    current_register_content
                }
                // set bit
                gpio_types::ModerTypes::GeneralPurposeOutputMode => {
                    current_register_content |= (0b01 as u32) << (self.pin_number * 2);
                    current_register_content
                }
                gpio_types::ModerTypes::AlternateFunctionMode => {
                    current_register_content |= (0b10 as u32) << self.pin_number * 2;
                    current_register_content
                }
                gpio_types::ModerTypes::AnalogMode => {
                    current_register_content |= (0b11 as u32) << self.pin_number * 2;
                    current_register_content
                }
            };
            unsafe {
                ptr::write_volatile(gpiox_moder as *mut u32, updated_register_content);
            }
        }
        // 11.4.6 GPIO port output data register (GPIOx_ODR) (x = A..H)
        pub fn set_odr(&self, odr_type: gpio_types::OutputState) {
            let gpiox_odr_offset = 0x14;
            let gpiox_odr = self.gpio_base_adress | gpiox_odr_offset;

            // 32 bit register
            let mut current_register_content: u32;

            unsafe {
                current_register_content = ptr::read_volatile(gpiox_odr as *const u32);
            }
            // clear bits
            // current_register_content &= !(0b1 as u32) << self.pin_number;

            let updated_register_content = match odr_type {
                gpio_types::OutputState::High => {
                    current_register_content |= (0b1 as u32) << self.pin_number;
                    current_register_content
                }
                gpio_types::OutputState::Low => {
                    // todo: does not work
                    current_register_content ^= !(0b0 as u32) << self.pin_number;
                    current_register_content
                }
            };
            unsafe {
                ptr::write_volatile(gpiox_odr as *mut u32, updated_register_content);
            }
        }
        // p237 -> 11.4.2 GPIO port output type register
        pub fn set_otyper(&self, output_type: gpio_types::OutputTypes) {
            let gpiox_otyper_offset = 0x04;
            let gpiox_otyper = self.gpio_base_adress | gpiox_otyper_offset;

            // 32 bit register
            let mut current_register_content: u32;

            unsafe {
                current_register_content = ptr::read_volatile(gpiox_otyper as *const u32);
            }

            let updated_register_content = match output_type {
                // clear bit
                gpio_types::OutputTypes::PushPull => {
                    current_register_content &= !(1 as u32) << self.pin_number;
                    current_register_content
                }
                // set bit
                gpio_types::OutputTypes::OpenDrain => {
                    current_register_content |= (1 as u32) << self.pin_number;
                    current_register_content
                }
            };
            unsafe {
                ptr::write_volatile(gpiox_otyper as *mut u32, updated_register_content);
            }
        }
        pub fn into_af(&self, af_number: u32) {
            let gpiox_af_offset = if self.pin_number < 8 { 0x20 } else { 0x24 };
            let gpiox_af = self.gpio_base_adress | gpiox_af_offset;

            // 32 bit register
            let mut current_register_content: u32;

            unsafe {
                current_register_content = ptr::read_volatile(gpiox_af as *const u32);
            }

            let mut pin = self.pin_number;
            if self.pin_number > 7 {
                pin -= 8;
            }

            current_register_content &= !(0xF as u32) << pin * 4;
            current_register_content |= af_number << pin * 4;

            unsafe {
                ptr::write_volatile(gpiox_af as *mut u32, current_register_content);
            }
        }
    }
}
