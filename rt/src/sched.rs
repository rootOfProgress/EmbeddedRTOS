pub mod scheduler {

    use crate::mem;
    const tasks_mem_location: [u32; 3] = [0x2000_0000, 0x2000_0004, 0x2000_0008];

    extern "C" {
        pub fn dispatch_task(stack_ptr: *mut u32);
    }

    pub fn set_up() {
        let current_task = mem::memory_handler::write(tasks_mem_location[2], 0x0000_0001);
    }

    pub fn context_switch() {
        let current_task = mem::memory_handler::read(tasks_mem_location[2]);
        if current_task == 0x1 {
            // go to 2nd...
            mem::memory_handler::write(tasks_mem_location[2], 0x0000_0002);
        } else {
            // go to 1st...
            mem::memory_handler::write(tasks_mem_location[2], 0x0000_0001);
        }
    }
}