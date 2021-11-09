pub mod systick {
    use core::ptr;
    const STK_CTRL: u32 = 0xE000_E010;
    const STK_LOAD: u32 = 0xE000_E014;
    const STK_VAL: u32 = 0xE000_E018;
    const FACTOR: u32 = 8000;

    #[repr(C)]
    pub struct STK {
        cycles_until_zero: u32,
    }

    impl STK {
        // sry little bit sloppy..
        pub fn set_up_systick(period_ms: u32) {
            let systick_reg = STK {
                cycles_until_zero: period_ms * FACTOR,
            };
            systick_reg.stk_load();
            systick_reg.stk_val_clr();
            systick_reg.stk_run();
        }
        fn stk_load(&self) {
            unsafe {
                let mut current_register_content = ptr::read_volatile(STK_LOAD as *const u32);

                current_register_content &= !(0x00FF_FFFF);

                ptr::write_volatile(
                    STK_LOAD as *mut u32,
                    current_register_content | self.cycles_until_zero,
                );
            }
        }
        fn stk_val_clr(&self) {
            unsafe {
                let current_register_content = ptr::read_volatile(STK_VAL as *const u32);
                ptr::write_volatile(
                    STK_VAL as *mut u32,
                    current_register_content & !(0x00FF_FFFF),
                );
            }
        }
        fn stk_run(&self) {
            unsafe {
                let mut existing_val = ptr::read_volatile(STK_CTRL as *const u32);

                existing_val &= !(0b111);
                existing_val |= (0b100) | (0b010) | (0b001);

                ptr::write_volatile(STK_CTRL as *mut u32, existing_val);
            }
        }
    }
}
