pub mod gpio_driver {
    const GPIOA_BASE: u32 = 0x48000000;

    enum OutputTypes {
        PushPull,
        OpenDrain,
    }

    struct GpioConfig {
        gpio_port: u32,
        gpio_pin: u8,
        gpio_port_mode: u32,
        gpio_output_type: u32,
        gpio_output_data_register: u32,
    }

    // p237 -> 11.4.2 GPIO port output type register
    fn set_otyper(port_base: u32, output_type: OutputTypes, pin: u8) -> u32 {
        let mut gpiox_otyper = port_base | 0x04;
        match output_type {
            // clear bit
            OutputTypes::PushPull => {
                gpiox_otyper &= !(1 as u32) << pin;
            }
            // set bit
            OutputTypes::OpenDrain => gpiox_otyper |= (1 as u32) << pin,
        }
        123
    }
}
