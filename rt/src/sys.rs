pub mod call_api {

    extern "C" {
        fn __trap(trap_id: &TrapMeta);
    }

    pub enum TrapReason {
        EnableRt,
        DisableRt,
        YieldTask,
        TerminateTask,
        Sleep,
        WriteStdOut
    }

    #[repr(C)]
    pub struct TrapMeta {
        pub id: TrapReason,
        pub payload: *const u8
    }

    pub fn println(str_start: &str) {
        let meta = TrapMeta {
            id: TrapReason::WriteStdOut,
            payload: str_start.as_ptr()
        };
        unsafe {
            __trap(&meta/* core::ptr::addr_of!(meta) as u32 */);
        }
    }

    pub fn enable_rt_mode() {
        let meta = TrapMeta {
            id: TrapReason::EnableRt,
            payload: 0x0 as *const u8
        };
        unsafe {
            __trap(&meta/* core::ptr::addr_of!(meta) as u32 */);
        }
    }

    pub fn disable_rt_mode() {
        let meta = TrapMeta {
            id: TrapReason::DisableRt,
            payload: 0x0 as *const u8
        };
        unsafe {
            __trap(&meta/* core::ptr::addr_of!(meta) as u32 */);
        }
    }
    
    pub fn sleep() {
        let meta = TrapMeta {
            id: TrapReason::Sleep,
            payload: 0x0 as *const u8
        };
        unsafe {
            __trap(&meta/* core::ptr::addr_of!(meta) as u32 */);
        }
    }

    pub fn yield_task() {
        let meta = TrapMeta {
            id: TrapReason::YieldTask,
            payload: 0x0 as *const u8
        };
        unsafe {
            __trap(&meta/* core::ptr::addr_of!(meta) as u32 */);
        }
    }

    pub fn terminate(){
        let meta = TrapMeta {
            id: TrapReason::TerminateTask,
            payload: 0x0 as *const u8
        };
        unsafe {
            __trap(&meta/* core::ptr::addr_of!(meta) as u32 */);
        }    
    }


}