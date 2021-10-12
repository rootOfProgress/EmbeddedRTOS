// stk = SysTick timer - ref: cortex-m4 p.246
pub mod stk {
    use core::ptr::{read_volatile, write_volatile};

    const SYSTICK_TIMER: u32 = 0xE000_E010;

    #[repr(C)]
    struct Systick {
        stk_ctrl: u32,
        stk_load: u32,
        stk_val: u32,
        stk_calib: u32,
    }

    pub struct SystemTimer {
        p: &'static mut Systick,
    }

    impl SystemTimer {
        pub fn take() -> SystemTimer {
            SystemTimer {
                p: unsafe { &mut *(SYSTICK_TIMER as *mut Systick) },
            }
        }

        // Sets the reload value
        // Reload value can be any value in the range 0x00000001-0x00FFFFFF.
        pub fn set_reload(self, load: u32) -> SystemTimer {
            if load <= 0x00FFFFFF {
                unsafe {
                    write_volatile(
                        &mut self.p.stk_load as *mut u32,
                        read_volatile(&mut self.p.stk_load) | load,
                    );
                }
            }
            self
        }

        // Sets TICKINT thus counting down to zero asserts the SysTick exception request
        // and also sets ENABLE thus enabling the counter
        pub fn enable(self) -> SystemTimer {
            unsafe {
                write_volatile(
                    &mut self.p.stk_ctrl as *mut u32,
                    read_volatile(&mut self.p.stk_ctrl) | 0b11,
                );
            }
            self
        }
    }
}
