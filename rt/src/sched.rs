pub mod task_control {
    use crate::dev::uart::{self, print_str};
    use crate::{dev::uart::print_dec, mem};
    use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
    enum TaskStates {
        READY,
        RUNNING,
        BLOCKED,
        TERMINATED,
    }

    #[repr(C)]
    #[repr(align(4))]
    pub struct TCB {
        sp: u32,
        state: TaskStates,
        pid: u32,
    }

    enum VecMeta {
        MAGIC,
        PREV,
        CURRENT,
        SIZE,
        FLUSH,
    }

    const TCB_START: u32 = 0x2000_0200;
    static HEAP_SIZE: AtomicU32 = AtomicU32::new(0);
    static CURRENT_TASK: AtomicU32 = AtomicU32::new(0);
    const TCB_SIZE: u32 = core::mem::size_of::<TCB>() as u32;

    const NUM_TASKS: u32 = 5;
    const VECTOR_START: u32 = 0x2000_0100;
    const ADR_OFFSET: u32 = 0x04;
    const DATA_START: u32 = 0x2000_0104;
    const BLOCK_SIZE: u8 = 0x08;

    fn get_vec_meta() -> (u8, u8, u8, u8) {
        let vec_meta = mem::memory_handler::read(VECTOR_START);
        (
            ((vec_meta & 0xff000000) >> 24) as u8, // magic
            ((vec_meta & 0x00ff0000) >> 16) as u8, // previous pos
            ((vec_meta & 0x0000ff00) >> 8) as u8,  // current pos
            ((vec_meta & 0x000000ff) >> 0) as u8,  // overall size
        )
    }

    fn write_meta(value: u32, v_type: VecMeta) {
        let vec_meta: u32 = mem::memory_handler::read(VECTOR_START);

        match v_type {
            VecMeta::MAGIC => mem::memory_handler::write(VECTOR_START, vec_meta | (value << 24)),
            VecMeta::PREV => mem::memory_handler::write(VECTOR_START, vec_meta | (value << 16)),
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
        let vec_meta = get_vec_meta();

        mem::memory_handler::write(
            (DATA_START + ADR_OFFSET) + ((vec_meta.1 as u32) * BLOCK_SIZE as u32),
            addr,
        );
    }

    pub fn current_task() -> (u32, u32) {
        let vec_meta = get_vec_meta();
        let task_adr = mem::memory_handler::read(
            (DATA_START + ADR_OFFSET) + ((vec_meta.2 as u32) * BLOCK_SIZE as u32),
        );
        let task_meta =
            mem::memory_handler::read((DATA_START) + ((vec_meta.2 as u32) * BLOCK_SIZE as u32));
        let task_mode = task_meta & 0x0000_FFFF;
        (task_adr, task_mode)
    }

    pub fn next_task() {
        let vec_meta = get_vec_meta();
        let vec_meta_blk: u32 = mem::memory_handler::read(VECTOR_START);

        mem::memory_handler::write(
            VECTOR_START,
            (vec_meta_blk & !(0x00FF_0000)) | (((vec_meta.2) as u32) << 16),
        );
        // size == current, go to 0
        if vec_meta.2 == (vec_meta.3 - 1) {
            mem::memory_handler::write(
                VECTOR_START,
                mem::memory_handler::read(VECTOR_START) & !(0x0000_FF00),
            );
        } else {
            mem::memory_handler::write(
                VECTOR_START,
                (mem::memory_handler::read(VECTOR_START) & !(0x0000_FF00))
                    | (((vec_meta.2 + 0b1) as u32) << 8),
            );
        }
    }

    pub fn insert_task(addr: u32, is_user: bool) {
        let vec_meta = get_vec_meta();
        let addr_task_meta = (BLOCK_SIZE * vec_meta.3) as u32 + DATA_START;
        let addr_task_ptr = addr_task_meta + 0x04;
        if is_user {
            mem::memory_handler::write(addr_task_meta, 0x00FFFFFF);
        } else {
            mem::memory_handler::write(addr_task_meta, 0x00FF0000);
        }
        mem::memory_handler::write(addr_task_ptr, addr);
        write_meta((vec_meta.3 + 0x01) as u32, VecMeta::SIZE);
    }

    pub fn print() {
        let tcb_size = core::mem::size_of::<TCB>();
        let tcb_location = unsafe { core::ptr::read_volatile(TCB_START as *const u32) };
        for tcb_addr in
            (TCB_START..(TCB_START + (NUM_TASKS * tcb_size as u32) as u32)).step_by(tcb_size)
        {
            let tcb = unsafe { &mut *(tcb_addr as *mut Option<TCB>) };
            match tcb {
                Some(tcb) => {
                    uart::print_dec(tcb.pid);
                    uart::print_str("\n\r");
                    unsafe {
                        asm! {"bkpt"}
                    }
                }
                None => {}
            }
        }
    }

    pub fn next_process() -> u32 {
        let current = CURRENT_TASK.fetch_add(1, Ordering::Relaxed) as u32;
        let next = (current + 1) % HEAP_SIZE.load(Ordering::Relaxed);
        let target_tcb_adress = (next * TCB_SIZE) + TCB_START;
        let tcb = unsafe { &mut *(target_tcb_adress as *mut Option<TCB>) };

        CURRENT_TASK.store(next, Ordering::Relaxed);

        match tcb {
            Some(t) => t.sp,
            None => 0x00,
        }
    }

    pub fn current_process() -> u32 {
        let current = CURRENT_TASK.load(Ordering::Relaxed) as u32;
        let entry_target = (current * TCB_SIZE) + TCB_START;
        let tcb = unsafe { &mut *(entry_target as *mut Option<TCB>) };
        match tcb {
            Some(t) => t.sp,
            None => 0x00,
        }
    }

    pub fn update_sp(new_sp: u32) {
        let current = CURRENT_TASK.load(Ordering::Relaxed) as u32;
        let entry_target = (current * TCB_SIZE) + TCB_START;
        let tcb = unsafe { &mut *(entry_target as *mut Option<TCB>) };
        *tcb = Some(TCB {
            sp: new_sp,
            state: TaskStates::READY,
            pid: current,
        });
    }

    pub fn insert(stack_pointer: u32, pid: u32) {
        let entry_target = (HEAP_SIZE.load(Ordering::Relaxed) as u32 * TCB_SIZE) + TCB_START;
        let tcb = unsafe { &mut *(entry_target as *mut Option<TCB>) };

        *tcb = Some(TCB {
            sp: stack_pointer,
            state: TaskStates::READY,
            pid,
        });

        HEAP_SIZE.fetch_add(1, Ordering::Relaxed);
    }

    pub fn set_up() {
        write_meta(0x0000_0000, VecMeta::FLUSH);
        write_meta(0xFF, VecMeta::MAGIC);
        write_meta(0x0, VecMeta::PREV);
        write_meta(0x0, VecMeta::CURRENT);
        write_meta(0x0, VecMeta::SIZE);
    }
}

pub mod scheduler {
    static USR_RUNS: AtomicBool = AtomicBool::new(false);
    use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

    extern "C" {
        pub fn __write_psp(addr: u32);
        fn __save_process_context();
        fn __load_process_context(addr: u32);
        fn __get_current_psp() -> u32;

        // pub fn __exec();
    }
    use crate::sched::task_control;

    use super::task_control::next_process;
    pub fn init_task_mng() {
        task_control::set_up();
    }

    pub fn queue_task(addr: u32, is_user: bool) {
        task_control::insert_task(addr, is_user);
    }

    pub fn immediate_start(addr: *const u32) {
        unsafe {
            __load_process_context(addr as u32);
        }
    }

    pub fn context_switch() {
        // if USR_RUNS.load(Ordering::Relaxed) {
            unsafe {
                __save_process_context();
                task_control::update_sp(__get_current_psp());
            }
        // }
        unsafe {
            let next = next_process();
            __load_process_context(next);
        }
        // USR_RUNS.store(true, Ordering::Relaxed);

        // store state and stack pointer of current process
        // task_control::next_task();
        // get next task in row
        // unsafe {
        //     asm! {"bkpt"}
        // }
    }
}
