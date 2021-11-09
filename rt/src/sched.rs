pub mod task_control {
    use core::sync::atomic::{AtomicU32, Ordering};
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

    const TCB_START: u32 = 0x2000_0200;
    static HEAP_SIZE: AtomicU32 = AtomicU32::new(0);
    pub static CURRENT_TASK: AtomicU32 = AtomicU32::new(0);
    const TCB_SIZE: u32 = core::mem::size_of::<TCB>() as u32;

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

    pub fn insert(stack_pointer: u32) -> u32 {
        let pid = HEAP_SIZE.load(Ordering::Relaxed);
        let entry_target = (pid * TCB_SIZE) + TCB_START;
        let tcb = unsafe { &mut *(entry_target as *mut Option<TCB>) };

        *tcb = Some(TCB {
            sp: stack_pointer,
            state: TaskStates::READY,
            pid,
        });

        HEAP_SIZE.fetch_add(1, Ordering::Relaxed);
        pid
    }
}

pub mod scheduler {

    extern "C" {
        pub fn __write_psp(addr: u32);
        fn __save_process_context();
        fn __load_process_context(addr: u32);
        fn __get_current_psp() -> u32;
    }
    use crate::sched::task_control;
    use super::task_control::{next_process};

    pub fn immediate_start(addr: *const u32) {
        unsafe {
            __load_process_context(addr as u32);
        }
    }

    pub fn context_switch() {
        unsafe {
            __save_process_context();
            task_control::update_sp(__get_current_psp());
            __load_process_context(next_process());
        }
    }
}
