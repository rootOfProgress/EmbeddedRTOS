//!
//! Collection of necessary devices for interrupt control.
//! Provides interrupt device instantiation, adjustment and en-/disable methods.
//!
use super::cpu;
pub mod systick {
    use super::cpu::{c_adresses, c_offsets, c_bitfields};
    use crate::mem::memory_handler::{read, write};
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
            let mut current_register_content = read(c_adresses::STK | c_offsets::stk::LOAD);
            current_register_content &= !(0x00FF_FFFF);
            write(c_adresses::STK | c_offsets::stk::LOAD, current_register_content | self.cycles_until_zero);
        }
        fn stk_val_clr(&self) {
            write(c_adresses::STK | c_offsets::stk::VAL, read(c_adresses::STK | c_offsets::stk::VAL) & !(0x00FF_FFFF));
        }
        fn stk_run(&self) {
            let mut existing_val = read(c_adresses::STK | c_offsets::stk::CTRL);

            existing_val &= !(0b111);
            existing_val |= (0b100) | (0b010) | (0b001);

            write(c_adresses::STK | c_offsets::stk::CTRL, existing_val);
        }
    }
    pub fn disable_systick() {
        write(c_adresses::STK | c_offsets::stk::CTRL, read(c_adresses::STK | c_offsets::stk::CTRL) & !(c_bitfields::stk::ENABLE));
    }
    pub fn enable_systick() {
        write(c_adresses::STK | c_offsets::stk::CTRL, read(c_adresses::STK | c_offsets::stk::CTRL) | c_bitfields::stk::ENABLE);
    }
}
