pub mod control {
    extern "C" {
        pub fn __get_current_msp();
        pub fn __get_current_psp();
        pub fn __save_process_context();
        pub fn __load_process_context();
        pub fn __write_psp(foo: u32);
        pub fn __exec();
    }
    pub fn read_main_stack_ptr() -> u32 {
        let mut msp_val: u32;
        unsafe {
            __get_current_msp();
            asm! ("mov {}, r0", out(reg) msp_val);
        }
        msp_val
    }
    pub fn read_process_stack_ptr() -> u32 {
        let mut psp_val: u32;
        unsafe {
            __get_current_psp();
            asm! ("mov {}, r0", out(reg) psp_val);
        }
        psp_val
    }
    pub fn save_proc_context() {
        unsafe {
            __save_process_context();
        }
    }
    pub fn load_proc_context() {
        unsafe {
            __load_process_context();
        }
    }
}
