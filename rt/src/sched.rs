pub mod scheduler {

    use crate::ctrl::control;
    use crate::mem;
    use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

    static is_in_user_mode: AtomicBool = AtomicBool::new(false);
    static msp_entry: AtomicU32 = AtomicU32::new(0x0000_0000);
    
    const TASK_CNT: u32 = 4;
    const TASK_COUNTER_ID: usize = 4;
    const TASK_ORDER: [u32; TASK_CNT as usize] = [1, 2, 3, 0];
    const TASKS_MEM_LOCATION: [u32; 5] = [
        0x2000_0008,
        0x2000_000C,
        0x2000_0010,
        0x2000_0014,
        0x2000_0018,
    ];



    pub fn set_up() {
        mem::memory_handler::write(TASKS_MEM_LOCATION[TASK_COUNTER_ID], 0x0000_0000)
    }

    pub fn set_msp_entry(v: u32) {
        msp_entry.store(v, Ordering::Relaxed);
    }
    pub fn get_msp_entry() -> u32 {
        msp_entry.load(Ordering::Relaxed)
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
        TASK_ORDER[current as usize]
    }

    pub fn subroutine(n: u8) {
        if n > 3 {
            return;
        }
        subroutine(n + 1);


        unsafe {
            // asm!("bkpt");
        }
    }

    pub fn context_switch() {

        // save usr state only if a usr process runs
        if is_in_user_mode.load(Ordering::Relaxed) {
            control::save_proc_context();
        } else {
            // save kernel context
        }

        // replace with "next"
        let current_task = mem::memory_handler::read(TASKS_MEM_LOCATION[4]);


        // update process stack pointer for current task
        mem::memory_handler::write(
            TASKS_MEM_LOCATION[current_task as usize],
            control::read_process_stack_ptr(),
        );

        // will be the "current_task" in the following interrupt
        mem::memory_handler::write(TASKS_MEM_LOCATION[TASK_COUNTER_ID], next(current_task));

        // set psp and pop of user registers
        run(mem::memory_handler::read(TASKS_MEM_LOCATION[4]));
    }
}
