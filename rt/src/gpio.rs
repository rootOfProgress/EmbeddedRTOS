pub mod gpio_types {
    
    pub enum OutputTypes {
        PushPull,
        OpenDrain,
    }
    
    pub enum ModerTypes {
        InputMode,
        GeneralPurposeOutputMode,
        AlternateFunctionMode,
        AnalogMode
    }
    
    pub enum OutputState {
        High,
        Low
    }
}

pub mod gpio_driver {
    use core::ptr;
    use super::gpio_types;

    // p52
    const GPIOA_BASE: u32 = 0x4800_0000;
    const GPIOB_BASE: u32 = 0x4800_0400;
    const GPIOC_BASE: u32 = 0x4800_0800;
    const GPIOE_BASE: u32 = 0x4800_1000;

    pub struct GpioX {
        gpio_base_adress: u32,
        pin_number: u8
    }

    impl GpioX {
        pub fn new(port_mnemonic: &str, pin_number: u8) -> GpioX {
            GpioX {
                gpio_base_adress: match port_mnemonic {
                    "A" => GPIOA_BASE,
                    "B" => GPIOB_BASE,
                    "C" => GPIOC_BASE,
                    "E" => GPIOE_BASE,
                    _ => GPIOA_BASE
                },
                pin_number
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
    }
}