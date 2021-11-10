pub mod tim2 {
    const TIM2_CR1: u32 = 0x4000_0000;
    use core::ptr;

    pub fn start_measurement() {
        unsafe {
            let existing_value = ptr::read_volatile(TIM2_CR1 as *mut u32);
            ptr::write_volatile(TIM2_CR1 as *mut u32, existing_value | 0b1);
        }
    }
    pub fn stop_measurement() {
        unsafe {
            let existing_value = ptr::read_volatile(TIM2_CR1 as *mut u32);
            ptr::write_volatile(TIM2_CR1 as *mut u32, existing_value & !(0b1));
        }
    }
    pub fn reset_timer() {
        // TIM2 RESET -> p 166
        let rcc_apb1rstr: u32 = 0x4002_1000 | 0x10;
        unsafe {
            let existing_value = ptr::read_volatile(rcc_apb1rstr as *mut u32);
            ptr::write_volatile(rcc_apb1rstr as *mut u32, existing_value | 0b1);
            let existing_value = ptr::read_volatile(rcc_apb1rstr as *mut u32);
            ptr::write_volatile(rcc_apb1rstr as *mut u32, existing_value & !(0b1));
        }
    }
    pub fn read_value() -> u32 {
        let timx_cnt: u32 = 0x4000_0000 | 0x24;
        unsafe { (ptr::read_volatile(timx_cnt as *mut u32) & !(0b1 << 31)) * 125 }
    }
}

// NOTE: ONLY FOR TESTPURPOSES, NO GENERIC USART DRIVER YET!
pub mod uart {
    const USART1_BASE: u32 = 0x4001_3800;
    use core::ptr;
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
            unsafe {
                // clk / 9600 baud
                ptr::write_volatile(usartx_brr as *mut u32, baudrate_divisor);
            }

            let usart1_cr1 = self.usart_base_adress;
            unsafe {
                let existing_value = ptr::read_volatile(usart1_cr1 as *mut u32);
                ptr::write_volatile(usart1_cr1 as *mut u32, existing_value | (0b1100));
                let existing_value = ptr::read_volatile(usart1_cr1 as *mut u32);
                ptr::write_volatile(usart1_cr1 as *mut u32, existing_value | (0b1));
            }
        }
    }

    // propably the world's worst and slowest function to print stupid integers on
    // a screen
    pub fn print_dec(mut dec: u32) {
        let usart2_tdr = 0x4001_3800 | 0x28;
        let usart2_isr = 0x4001_3800 | 0x1C;
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
        let usart2_tdr = 0x4001_3800 | 0x28;
        let usart2_isr = 0x4001_3800 | 0x1C;

        unsafe {
            while *ptr_start != b'\0' {
                ptr::write_volatile(usart2_tdr as *mut u32, *ptr_start as u32);
                ptr_start = ptr_start.add(1);
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
