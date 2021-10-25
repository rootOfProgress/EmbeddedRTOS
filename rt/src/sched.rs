pub mod scheduler {


    use crate::ctrl::control;
    use crate::mem;
    use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

    static is_in_user_mode: AtomicBool = AtomicBool::new(false);
    static msp_entry: AtomicU32 = AtomicU32::new(0x0000_0000);
    
    const TASK_CNT: u32 = 4;
    const VECTOR_START: u32 = 0x2000_0100;
    const VECTOR_MAGIC: u32 = VECTOR_START | 0x00;
    // const VECTOR_MAGIC: u32 = VECTOR_START | 0x00;
    const VECTOR_SIZE: u32 = 0xC;
    const VECTOR_CURRENT: u32 = 0x8;
    const DATA_START: u32 = 0x2000_0110;
    const BLOCK_SIZE: u8 = 0x20;

    enum VecMeta {
        MAGIC,
        NOTASSIGNED,
        CURRENT,
        SIZE
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

    pub fn get_vec_meta() -> (u8,u8,u8,u8)  {
        let vec_meta = mem::memory_handler::read(VECTOR_START);
        (
            (vec_meta & 0xff000000 >> 24) as u8, // magic
            (vec_meta & 0x00ff0000 >> 16) as u8, // not assigned
            (vec_meta & 0x0000ff00 >>  8) as u8, // current pos
            (vec_meta & 0x000000ff >>  0) as u8  // overall size
        )
    }

    fn write_meta(value: u32, v_type: VecMeta) {
        let vec_meta: u32 = mem::memory_handler::read(VECTOR_START);

        match v_type {
            VecMeta::MAGIC => mem::memory_handler::write(VECTOR_START, vec_meta | (value << 24)),
            VecMeta::NOTASSIGNED => mem::memory_handler::write(VECTOR_START, vec_meta | (value << 16)),
            VecMeta::CURRENT => mem::memory_handler::write(VECTOR_START, vec_meta | (value << 8)),
            VecMeta::SIZE => mem::memory_handler::write(VECTOR_START, vec_meta | (value << 0))
        }
    }

    fn write_magic(magic: u32) {
        let mut vec_meta = mem::memory_handler::read(VECTOR_START);
        vec_meta |= magic << 24;
        mem::memory_handler::write(VECTOR_START, vec_meta);
    }

    fn write_dummy(d: u32) {
        let mut vec_meta = mem::memory_handler::read(VECTOR_START);
        vec_meta |= d << 16;
        mem::memory_handler::write(VECTOR_START, vec_meta);
    }

    fn write_pos(pos: u32) {
        let mut vec_meta = mem::memory_handler::read(VECTOR_START);
        vec_meta |= pos << 8;
        mem::memory_handler::write(VECTOR_START, vec_meta);
    }

    fn write_size(s: u32) {
        let mut vec_meta = mem::memory_handler::read(VECTOR_START);
        vec_meta |= s << 0;
        mem::memory_handler::write(VECTOR_START, vec_meta);
    }

    fn flush() {
        let mut vec_meta = mem::memory_handler::read(VECTOR_START);
        vec_meta = 0x0000_0000;
        mem::memory_handler::write(VECTOR_START, vec_meta);
    }
    pub fn insert_task(addr: u32, mode: u8, state: u8) {
        // let vec_size = mem::memory_handler::read(VECTOR_START | VECTOR_SIZE);

    }


    pub fn set_up() {
        flush();
        write_magic(0xFF);
        write_dummy(0x11);
        write_pos(0x00);
        write_size(0x00);
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
            asm!("bkpt");
        }
    }

    pub fn context_switch() {

        // save usr state only if a usr process runs
        if is_in_user_mode.load(Ordering::Relaxed) {
            control::save_proc_context();
        } else {
            unsafe {
                asm!("bkpt");
            }
           is_in_user_mode.store(true, Ordering::Relaxed);
           return;
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
