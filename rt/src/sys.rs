///
/// WORK IN PROGRESS
/// 
pub mod call_api {

    extern "C" {
        fn __trap(trap_id: &TrapMeta);
    }

    #[repr(C)]
    pub enum TrapReason {
        EnableRt,
        DisableRt,
        YieldTask,
        TerminateTask,
        Sleep,
        WriteStdOut
    }

    #[repr(C)]
    #[repr(align(4))]
    pub struct TrapMeta {
        pub id: TrapReason,
        pub payload: *const u32
    }

    pub fn println(str_start: &str) {
        let meta = TrapMeta {
            id: TrapReason::WriteStdOut,
            payload: str_start.as_ptr() as *const u32
        };
        unsafe {
            __trap(&meta);
        }
    }

    pub fn enable_rt_mode() {
        let meta = TrapMeta {
            id: TrapReason::EnableRt,
            payload: 0x0 as *const u32
        };
        unsafe {
            __trap(&meta);
        }
    }

    pub fn disable_rt_mode() {
        let meta = TrapMeta {
            id: TrapReason::DisableRt,
            payload: 0x0 as *const u32
        };
        unsafe {
            __trap(&meta);
        }
    }
    
    pub fn sleep(time_to_sleep: u32) {
        let meta = TrapMeta {
            id: TrapReason::Sleep,
            payload: &time_to_sleep as *const u32
        };
        unsafe {
            __trap(&meta);
        }
    }

    pub fn yield_task() {
        let meta = TrapMeta {
            id: TrapReason::YieldTask,
            payload: 0x0 as *const u32
        };
        unsafe {
            __trap(&meta);
        }
    }

    pub fn terminate(){
        let meta = TrapMeta {
            id: TrapReason::TerminateTask,
            payload: 0x0 as *const u32
        };
        unsafe {
            __trap(&meta);
        }    
    }


}