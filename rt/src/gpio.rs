pub mod gpio_driver {
    let GPIOA_BASE = 0x48000000;

    enum OutputTypes {
        PUSH_PULL,
        OPEN_DRAIN
    }

    struct GpioConfig {
        gpio_port: u32,
        gpio_pin: u8,
        gpio_port_mode: u32,
        gpio_output_type: u32,
        gpio_output_data_register: u32
    }

    // p237 -> 11.4.2 GPIO port output type register
    fn set_otyper(port_base: u32, type: OutputTypes, pin: u8) -> u32 {

        let gpiox_otyper = port_base | 0x04;
        match type {
            // clear bit
            OutputTypes::PUSH_PULL => { gpiox_otyper &= !(1 as u32 << pin) },
            // set bit
            OutputTypes::OPEN_DRAIN => { gpiox_otyper |= (1 as u32 << pin) }
        }
    }
}