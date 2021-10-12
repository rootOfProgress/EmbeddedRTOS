pub mod systick {
    use core::ptr;
    const STK_BASE: u32 = 0xE000_E010;
    const FACTOR: u32 = 8000;

    #[repr(C)]
    pub struct STK {
        stk_ctrl: u32,
        stk_load: u32,
        stk_val: u32,
        stk_calib: u32
    }

    impl STK {
        pub fn set_up_systick(period_ms: u32)  {
            let systick_reg = unsafe { &mut *(STK_BASE as *mut STK) };
            systick_reg.stk_load(period_ms);
        }
        fn stk_load(&self, period_ms: u32) {
            unsafe {
                let mut current_register_content = ptr::read_volatile(self.stk_load as *const u32);
        
                // clear out first 24bit's
                let mut updated_register_content = current_register_content & !(0x00FF_FFFF);
        
                let cycles_until_zero = period_ms * FACTOR;
                updated_register_content |= cycles_until_zero;
                ptr::write_volatile(self.stk_load as *mut u32, current_register_content);
            }
        }
        fn stk_val_clr(&self) {
            unsafe {
                let mut current_register_content = ptr::read_volatile(self.stk_val as *const u32);
        
                // clear out first bit (LSB)
                let updated_register_content = current_register_content & !(0x00FF_FFFF);
        
                ptr::write_volatile(self.stk_val as *mut u32, updated_register_content);
            }
        }
        fn stk_run(&self) {
            unsafe {
                let mut existing_val = ptr::read_volatile(self.stk_val as *const u32);
        
                // clear out first three bits (LSB)
                existing_val &= !(0b111);
                existing_val |= (0b100) | (0b010) | (0b001);
        
                ptr::write_volatile(self.stk_val as *mut u32,existing_val);
            }
        }
    }
}