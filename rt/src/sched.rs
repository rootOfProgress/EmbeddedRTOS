pub mod task_control {
    use crate::mem;

    enum VecMeta {
        MAGIC,
        NOTASSIGNED,
        CURRENT,
        SIZE,
        FLUSH,
    }

    const VECTOR_START: u32 = 0x2000_0100;
    const ADR_OFFSET: u32 = 0x04;
    const DATA_START: u32 = 0x2000_0104;
    const BLOCK_SIZE: u8 = 0x08;

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
        let vec_meta = get_vec_meta();
        let mut foo = vec_meta.2 as u32;
        if foo == 0 {
            foo = 3;
        } else {
            foo -= 1;
        }
        mem::memory_handler::write(
            (DATA_START + ADR_OFFSET) + ((foo as u32) * BLOCK_SIZE as u32),
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

    pub fn set_up() {
        write_meta(0x0000_0000, VecMeta::FLUSH);
        write_meta(0xFF, VecMeta::MAGIC);
        write_meta(0xAB, VecMeta::NOTASSIGNED);
        write_meta(0x0, VecMeta::CURRENT);
        write_meta(0x0, VecMeta::SIZE);
    }
}

pub mod scheduler {

    use crate::sched::task_control;

    use crate::ctrl::control;
    use core::sync::atomic::{AtomicU32, Ordering};

    static MSP_ENTRY: AtomicU32 = AtomicU32::new(0x0000_0000);

    pub fn init_task_mng() {
        task_control::set_up();
    }

    pub fn queue_task(addr: u32, is_user: bool) {
        task_control::insert_task(addr, is_user);
    }

    pub fn set_msp_entry(v: u32) {
        MSP_ENTRY.store(v, Ordering::Relaxed);
    }
    pub fn get_msp_entry() -> u32 {
        MSP_ENTRY.load(Ordering::Relaxed)
    }

    pub fn run(task_addr: u32) {
        unsafe {
            control::__write_psp(task_addr);
        }
    }

    pub fn context_switch() {
        let (task_addr, task_mode) = task_control::current_task();

        if task_mode == 0xFFFF {
            run(task_addr);
        } 
        task_control::next_task();
    }
}
