pub mod scheduler {

    use crate::mem;
    use core::ptr;
    use core::sync::atomic::{AtomicU8, Ordering};
    static foo: AtomicU8 = AtomicU8::new(0);
    use crate::ctrl::control;
    const tasks_mem_location: [u32; 3] = [0x2000_0000, 0x2000_0004, 0x2000_0008];

    extern "C" {
        pub fn dispatch_task(stack_ptr: *mut u32);
    }

    pub fn set_up() {
        mem::memory_handler::write(tasks_mem_location[2], 0x0000_0000)
    }

    pub fn init(task_number: usize, addr: u32) {
        unsafe {
            mem::memory_handler::write(tasks_mem_location[task_number], addr)
        }
    }

    pub fn run(task_number: u32) {
        unsafe {
            let task_addr = mem::memory_handler::read(tasks_mem_location[task_number as usize]);
            control::__write_psp(task_addr);
            control::__load_process_context();
            unsafe {
                // asm!("bkpt")
            }
            control::__exec();
        }
    }

    pub fn context_switch() {

        control::save_proc_context();
        let current_task = mem::memory_handler::read(tasks_mem_location[2]);
        unsafe {
            mem::memory_handler::write(tasks_mem_location[current_task as usize], control::read_process_stack_ptr());
            // asm!("bkpt");
        }

        if (current_task == 0) {
            mem::memory_handler::write(tasks_mem_location[2], 0x0000_0001)
        } else {
            mem::memory_handler::write(tasks_mem_location[2], 0x0000_0000)
        }
        
        run(mem::memory_handler::read(tasks_mem_location[2]));

    }
}