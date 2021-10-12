#![feature(asm)]
pub mod systick {
    use core::ptr;
    const STK_BASE: u32 = 0xE000_E010;
    const FACTOR: u32 = 8000;

    // p 248
    pub fn setup_st_reload() {
        let stk_load_base: u32 = 0xE000_E014;
        unsafe {
            let mut existing_val = ptr::read_volatile(stk_load_base as *const u32);

            // clear out first bit (LSB)
            for i in 0..24 {
                existing_val &= !(1 << i);
            }

            for i in 0..23 {
                existing_val |= 0b1 << i;
            }

            ptr::write_volatile(stk_load_base as *mut u32,existing_val);
        }
    }

    // clears current cnt value from garbage values
    pub fn stk_val_clr() {
        let stk_load_base: u32 = 0xE000_E018;
        unsafe {
            let mut existing_val = ptr::read_volatile(stk_load_base as *const u32);

            // clear out first bit (LSB)
            for i in 0..24 {
                existing_val &= !(1 << i);
            }

            ptr::write_volatile(stk_load_base as *mut u32,existing_val);
        }
    }



    pub fn stk_ctrl() {
        // see p 246 reg boundaries cortex-m4 cpu
        let stk_ctrl_base: u32 = 0xE000_E010;
                        unsafe {
                    asm! {"bkpt"}
                }
        unsafe {
            let mut existing_val = ptr::read_volatile(stk_ctrl_base as *const u32);

            // clear out first bit (LSB)
            existing_val &= !(1 << 0);

            // clear out second bit (LSB)
            existing_val &= !(1 << 1);

            // clear out second bit (LSB)
            existing_val &= !(1 << 2);

            // set bit 0 & 1 to 0b01 (see p. 247, cortex-m4 ref man)
            existing_val |= (0b100) | (0b010) | (0b001);

            ptr::write_volatile(stk_ctrl_base as *mut u32,existing_val);
        }
    }

    // #[repr(C)]
    // pub struct STK {
    //     stk_ctrl: u32,
    //     stk_load: u32,
    //     stk_val: u32,
    //     stk_calib: u32
    // }

    // impl STK {
    //     pub fn set_up_systick(period_ms: u32)  {
    //         let systick_reg = unsafe { &mut *(STK_BASE as *mut STK) };
    //         systick_reg.stk_load_val(period_ms);
    //         systick_reg.stk_val_clr();
    //         systick_reg.stk_run();
    //     }
    //     fn stk_load_val(&self, period_ms: u32) {
    //         let stk_load_base: u32 = 0xE000_E014;
    //         unsafe {
    //             let mut existing_val = ptr::read_volatile(stk_load_base as *const u32);
        
    //             // clear out first bit (LSB)
    //             for i in 0..24 {
    //                 existing_val &= !(1 << i);
    //             }
        
    //             for i in 0..23 {
    //                 existing_val |= 0b1 << i;
    //             }
        
    //             ptr::write_volatile(stk_load_base as *mut u32,existing_val);
    //         }
    //         // unsafe {
    //         //     let mut current_register_content = ptr::read_volatile(self.stk_load as *const u32);
    //         //     unsafe {
    //         //         asm! {"bkpt"}
    //         //     }
        
    //         //     // clear out first 24bit's
    //         //     let mut updated_register_content = current_register_content & !(0x00FF_FFFF);
        
    //         //     let cycles_until_zero = period_ms * FACTOR;
    //         //     updated_register_content |= cycles_until_zero;
    //         //     ptr::write_volatile(self.stk_load as *mut u32, current_register_content);
    //         // }
    //     }
    //     fn stk_val_clr(&self) {
    //         unsafe {
    //             // unsafe {
    //             //     asm!(
    //             //         "bkpt"
    //             //     )
    //             // }
    //             let mut existing_val = ptr::read_volatile(0xE000E018 as *const u32);
        
    //             // clear out first bit (LSB)
    //             // let updated_register_content = current_register_content & !(0x00FF_FFFF);
    //             // clear out first bit (LSB)
    //             for i in 0..24 {
    //                 existing_val &= !(1 << i);
    //             }
    //             ptr::write_volatile(0xE000E018 as *mut u32, existing_val);
    //         }
    //     }
    //     fn stk_run(&self) {
    //         // see p 246 reg boundaries cortex-m4 cpu
    //         let stk_ctrl_base: u32 = 0xE000_E010;
    //         unsafe {
    //             let mut existing_val = ptr::read_volatile(stk_ctrl_base as *const u32);

    //             // clear out first bit (LSB)
    //             existing_val &= !(1 << 0);
        
    //             // clear out second bit (LSB)
    //             existing_val &= !(1 << 1);
        
    //             // clear out second bit (LSB)
    //             existing_val &= !(1 << 2);
        
    //             // set bit 0 & 1 to 0b01 (see p. 247, cortex-m4 ref man)
    //             existing_val |= (0b100) | (0b010) | (0b001);
        
    //             ptr::write_volatile(stk_ctrl_base as *mut u32,existing_val);
    //         }
    //     }
    // }
}