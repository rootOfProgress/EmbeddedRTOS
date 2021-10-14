pub mod scheduler {
    #[no_mangle]
    extern "C" {
        pub fn dispatch_task(stack_ptr: *mut u32);
    }
}