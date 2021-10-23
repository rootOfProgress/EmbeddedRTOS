pub mod scheduler {

    use crate::ctrl::control;
    use crate::mem;

    const TASK_CNT: u32 = 1;
    const TASK_COUNTER_ID: usize = 4;
    // const TASK_ORDER: [u32; TASK_CNT as usize] = [1, 2, 3, 0];
    const TASKS_MEM_LOCATION: [u32; 5] = [
        0x2000_0000,
        0x2000_0004,
        0x2000_0008,
        0x2000_000C,
        0x2000_0010,
    ];

    pub fn set_up() {
        mem::memory_handler::write(TASKS_MEM_LOCATION[TASK_COUNTER_ID], 0x0000_0000)
    }

    pub fn init(task_number: usize, addr: u32) {
        mem::memory_handler::write(TASKS_MEM_LOCATION[task_number], addr)
    }

    pub fn run(task_number: u32) {
        unsafe {
            let task_addr = mem::memory_handler::read(TASKS_MEM_LOCATION[task_number as usize]);
            control::__write_psp(task_addr);
            control::__load_process_context();
        }
    }

    fn next(current: u32) -> u32 {
        // TASK_ORDER[current as usize]
        0
    }

    pub fn context_switch() {
        control::save_proc_context();
        let current_task = mem::memory_handler::read(TASKS_MEM_LOCATION[4]);

        mem::memory_handler::write(
            TASKS_MEM_LOCATION[current_task as usize],
            control::read_process_stack_ptr(),
        );
        mem::memory_handler::write(TASKS_MEM_LOCATION[TASK_COUNTER_ID], next(current_task));

        run(mem::memory_handler::read(TASKS_MEM_LOCATION[4]));
    }
}
