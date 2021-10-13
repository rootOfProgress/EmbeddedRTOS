pub mod bus {
    use core::ptr::{read_volatile, write_volatile};

    const RCC_AHBENR: u32 = 0x4002_1000 | 0x14;

    pub struct AHB1 {
        rcc: &'static mut RCC,
    }

    impl AHB1 {
        pub fn take() -> AHB1 {
            AHB1 {
                rcc: unsafe { &mut *(RCC_AHBENR as *mut RCC) },
            }
        }

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
            // IOPAEN p.166 "io port e enable"
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
        pub fn take() -> AHB2 {
            AHB2 {
                gpioe: unsafe { &mut *(GPIOE_BASE as *mut GPIO) },
            }
        }

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
