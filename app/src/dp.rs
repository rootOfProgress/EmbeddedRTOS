pub mod bus {
    use core::{
        mem::replace,
        ptr::{read_volatile, write_volatile},
    };

    const RCC_AHBENR: u32 = 0x4002_1000 | 0x14;

    pub static mut PERIPHERALS: Peripherals = Peripherals {
        serial: Some(Serial),
    };

    pub struct Peripherals {
        serial: Option<Serial>,
    }

    impl Peripherals {
        pub fn take_serial(&mut self) -> Serial {
            let p = replace(&mut self.serial, None);
            p.unwrap()
        }
    }

    pub struct Serial;

    impl Serial {
        pub fn ahb1(&self) -> AHB1 {
            AHB1 {
                rcc: unsafe { &mut *(RCC_AHBENR as *mut RCC) },
            }
        }

        pub fn ahb2(&self) -> AHB2 {
            AHB2 {
                gpioe: unsafe { &mut *(GPIOE_BASE as *mut GPIO) },
            }
        }
    }

    pub struct AHB1 {
        rcc: &'static mut RCC,
    }

    impl AHB1 {
        pub fn rcc(&mut self, f: fn(&mut RCC) -> &mut RCC) {
            f(self.rcc);
        }
    }

    #[repr(C)]
    pub struct RCC {
        ahbenr: u32,
    }

    impl RCC {
        pub fn iopeen(&mut self) -> &mut RCC {
            // IOPEEN p.166 "io port e enable"
            unsafe {
                write_volatile(
                    &mut self.ahbenr as *mut u32,
                    read_volatile(&mut self.ahbenr) | (1 << 21),
                )
            };
            self
        }
    }

    const GPIOE_BASE: u32 = 0x4800_1000;

    pub struct AHB2 {
        gpioe: &'static mut GPIO,
    }

    impl AHB2 {
        pub fn gpioe(self) -> &'static mut GPIO {
            self.gpioe
        }
    }

    #[repr(C)]
    pub struct GPIO {
        pub moder: u32,
        pub otyper: u32,
        pub ospeedr: u32,
        pub pupdr: u32,
        pub idr: u32,
        pub odr: u32,
        pub bsrr: u32,
        pub lckr: u32,
        pub afrl: u32,
        pub afrh: u32,
        pub brr: u32,
    }
}
