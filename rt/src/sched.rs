pub mod scheduler {

    use crate::ctrl::control;
    use crate::mem;
    use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

    static is_user_task: AtomicBool = AtomicBool::new(false);
    static usr_is_running: AtomicBool = AtomicBool::new(false);
    static msp_entry: AtomicU32 = AtomicU32::new(0x0000_0000);

    const TASK_CNT: u32 = 4;
    const VECTOR_START: u32 = 0x2000_0100;
    const VECTOR_MAGIC: u32 = VECTOR_START | 0x00;
    // const VECTOR_MAGIC: u32 = VECTOR_START | 0x00;
    const VECTOR_SIZE: u32 = 0xC;
    const VECTOR_CURRENT: u32 = 0x8;
    const ADR_OFFSET: u32 = 0x04;
    const DATA_START: u32 = 0x2000_0104;
    const BLOCK_SIZE: u8 = 0x08;

    enum VecMeta {
        MAGIC,
        NOTASSIGNED,
        CURRENT,
        SIZE,
        FLUSH,
    }

    enum State {
        RUNNING, // 0
        WAITING, // 1
        READY,   // 2
    }

    enum TaskMeta {
        MODE,
        State,
    }
    const TASK_COUNTER_ID: usize = 4;
    const TASK_ORDER: [u32; TASK_CNT as usize] = [1, 2, 3, 0];
    const TASKS_MEM_LOCATION: [u32; 5] = [
        0x2000_0008,
        0x2000_000C,
        0x2000_0010,
        0x2000_0014,
        0x2000_0018,
    ];

    pub fn is_usr() -> bool {
        is_user_task.load(Ordering::Relaxed)
    } 
    pub fn running() -> bool {
        usr_is_running.load(Ordering::Relaxed)
    } 
    fn get_vec_meta() -> (u8, u8, u8, u8) {
        let vec_meta = mem::memory_handler::read(VECTOR_START);
        (
            ((vec_meta & 0xff000000) >> 24) as u8, // magic
            ((vec_meta & 0x00ff0000) >> 16) as u8, // not assigned
            ((vec_meta & 0x0000ff00) >> 8) as u8,  // current pos
            ((vec_meta & 0x000000ff) >> 0) as u8,  // overall size
        )
    }

    fn write_meta(value: u32, v_type: VecMeta) {
        let vec_meta: u32 = mem::memory_handler::read(VECTOR_START);

        match v_type {
            VecMeta::MAGIC => mem::memory_handler::write(VECTOR_START, vec_meta | (value << 24)),
            VecMeta::NOTASSIGNED => {
                mem::memory_handler::write(VECTOR_START, vec_meta | (value << 16))
            }
            VecMeta::CURRENT => mem::memory_handler::write(VECTOR_START, vec_meta | (value << 8)),
            VecMeta::SIZE => {
                mem::memory_handler::write(VECTOR_START, (vec_meta & !(0xFF)) | (value << 0))
            }
            VecMeta::FLUSH => mem::memory_handler::write(VECTOR_START, value),
        }
    }

    pub fn update_tasks_ptr(addr: u32) {
        if addr == 0x0000_0000 {
            return;
        }
        unsafe {
            // asm!("bkpt");
        }
        let vec_meta = get_vec_meta();
        let mut foo = vec_meta.2 as u32;
        if foo == 0 {
            foo = 3;
        } else {
            foo -= 1;
        }
        mem::memory_handler::write((DATA_START + ADR_OFFSET) + ((foo as u32) * BLOCK_SIZE as u32), addr);
    }

    fn current_task() -> (u32, u32) {
        let vec_meta = get_vec_meta();
        let task_adr = mem::memory_handler::read((DATA_START + ADR_OFFSET) + ((vec_meta.2 as u32) * BLOCK_SIZE as u32));
        let task_meta = mem::memory_handler::read((DATA_START) + ((vec_meta.2 as u32) * BLOCK_SIZE as u32));
        let task_mode = task_meta & 0x0000_FFFF;
        (task_adr, task_mode)
    }

    fn next_task() {
        let vec_meta = get_vec_meta();
        let vec_meta_blk: u32 = mem::memory_handler::read(VECTOR_START);
        // size == current, go to 0
        if vec_meta.2 == (vec_meta.3 - 1) {
            mem::memory_handler::write(VECTOR_START, vec_meta_blk & !(0xFF << 8));
        } else {            
            mem::memory_handler::write(
                VECTOR_START,
                vec_meta_blk & !(0xFF00) | (((vec_meta.2 + 0x01) as u32) << 8) as u32,
            );
        }
    }

    pub fn insert_task(addr: u32) {
        let vec_meta = get_vec_meta();
        let addr_task_meta = (BLOCK_SIZE * vec_meta.3) as u32 + DATA_START;
        let addr_task_ptr = addr_task_meta + 0x04;
        mem::memory_handler::write(addr_task_meta, 0x00FFFFFF);
        mem::memory_handler::write(addr_task_ptr, addr);
        write_meta((vec_meta.3 + 0x01) as u32, VecMeta::SIZE);
    }

    pub fn set_up() {
        write_meta(0x0000_0000, VecMeta::FLUSH);
        write_meta(0xFF, VecMeta::MAGIC);
        write_meta(0xAB, VecMeta::NOTASSIGNED);
        write_meta(0x0, VecMeta::CURRENT);
        write_meta(0x0, VecMeta::SIZE);
        // unsafe {
        //     asm! {"bkpt"}
        // }
        // mem::memory_handler::write(VECTOR_START, 0xFF00_0000);
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

    pub fn run(task_addr: u32) {
        unsafe {
            // let task_addr = mem::memory_handler::read(TASKS_MEM_LOCATION[task_number as usize]);
            control::__write_psp(task_addr);
            // control::__load_process_context();
        }
    }

    pub fn context_switch() {
        // mem::memory_handler::write(
        //     TASKS_MEM_LOCATION[current_task as usize],
        //     control::read_process_stack_ptr(),
        // );
        // update_tasks_ptr(control::read_process_stack_ptr());
        // schedule current
        // control::save_proc_context();
        let (task_addr, task_mode) = current_task();
        if task_mode == 0xFFFF {
            is_user_task.store(true, Ordering::Relaxed);
            usr_is_running.store(true, Ordering::Relaxed);
            run(task_addr);
        } else {
            is_user_task.store(false, Ordering::Relaxed);
        }
        next_task();
        let n = get_vec_meta();
        // save usr state only if a usr process runs
        // if is_in_user_mode.load(Ordering::Relaxed) {
        //     control::save_proc_context();
        // } else {
        //     unsafe {
        //         asm!("bkpt");
        //     }
        //     is_in_user_mode.store(true, Ordering::Relaxed);
        //     return;
        // }

        // replace with "next"
        // let current_task = mem::memory_handler::read(TASKS_MEM_LOCATION[4]);

        // update process stack pointer for current task
        // mem::memory_handler::write(
        //     TASKS_MEM_LOCATION[current_task as usize],
        //     control::read_process_stack_ptr(),
        // );

        // will be the "current_task" in the following interrupt
        // mem::memory_handler::write(TASKS_MEM_LOCATION[TASK_COUNTER_ID], next(current_task));

        // set psp and pop of user registers
        // let n = next();
        // update_tasks_ptr(get_msp_entry());

        // run(next());
    }
}
