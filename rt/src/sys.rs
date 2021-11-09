pub mod call_api {

    extern "C" {
        fn __trap(trap_id: u32);
    }

    pub enum TrapReason {
        YieldTask,
        Sleep
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
}