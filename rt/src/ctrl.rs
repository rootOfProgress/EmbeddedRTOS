pub mod control {
    use crate::mem;

    extern "C" {
        pub fn get_current_msp();
        pub fn get_current_psp();
        pub fn _save_process_context();
        pub fn _load_process_context();

    }
    pub fn read_main_stack_ptr() -> u32 {
        let mut msp_val: u32;
        unsafe {
            get_current_msp();
            asm! ("mov {}, r0", out(reg) msp_val);
            asm! {"bkpt"}
        }
        msp_val
    }
    pub fn read_process_stack_ptr() -> u32 {
        let mut psp_val: u32;
        unsafe {
            get_current_psp();
            asm! ("mov {}, r0", out(reg) psp_val);
            asm! {"bkpt"}
        }
        psp_val
    }
    pub fn save_proc_context() {
        unsafe {
            _save_process_context();
            asm! {"bkpt"}
        }
    }
    pub fn load_proc_context() {
        unsafe {
            _load_process_context();
            asm! {"bkpt"}
        }
    }
    pub fn write_stack_ptr(foo: *mut u32) {

    }
}