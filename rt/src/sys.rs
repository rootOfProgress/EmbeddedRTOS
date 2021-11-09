pub mod call_api {

    extern "C" {
        fn __trap(trap_id: u32);
    }

    pub enum TrapReason {
        YieldTask,
        Sleep
    }

    pub fn enable_rt_mode() {
        unsafe {
            __trap(3);
        }
    }

    pub fn disable_rt_mode() {
        unsafe {
            __trap(4);
        }
    }
    
    pub fn sleep() {
        unsafe {
            __trap(0);
        }
    }

    pub fn yield_task() {
        unsafe {
            __trap(1);
        }
    }

    pub fn terminate(){
        unsafe {
            __trap(2);
        }       
    }


}