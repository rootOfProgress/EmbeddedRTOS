//!
//! Collection of necessary devices for interrupt control.
//! Provides interrupt device instantiation, adjustment and en-/disable methods.
//!

pub mod systick {
    use crate::mem::memory_handler::{read, write};
    const STK_CTRL: u32 = 0xE000_E010;
    const STK_LOAD: u32 = 0xE000_E014;
    const STK_VAL: u32 = 0xE000_E018;
    const FACTOR: u32 = 8000;

    #[repr(C)]
    pub struct STK {
        cycles_until_zero: u32,
    }

    impl STK {
        pub fn set_up_systick(period_ms: u32) {
            let systick_reg = STK {
                cycles_until_zero: period_ms * FACTOR,
            };
            systick_reg.stk_load();
            systick_reg.stk_val_clr();
            systick_reg.stk_run();
        }
        fn stk_load(&self) {
            let mut current_register_content = read(STK_LOAD);
            current_register_content &= !(0x00FF_FFFF);
            write(STK_LOAD, current_register_content | self.cycles_until_zero);
        }
        fn stk_val_clr(&self) {
            write(STK_VAL, read(STK_VAL) & !(0x00FF_FFFF));
        }
        fn stk_run(&self) {
            let mut existing_val = read(STK_CTRL);

            existing_val &= !(0b111);
            existing_val |= (0b100) | (0b010) | (0b001);

            write(STK_CTRL, existing_val);
        }
    }
    pub fn disable_systick() {
        write(STK_CTRL, read(STK_CTRL) & !(0b1));
    }
    pub fn enable_systick() {
        write(STK_CTRL, read(STK_CTRL) | 0b1);
    }
}
