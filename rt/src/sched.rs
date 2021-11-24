//!
//! Contains the process table and appropriate
//! methods to control task switching. 
//!

pub mod task_control {
    use core::sync::atomic::{AtomicU32, Ordering};

    /// Represents the possible states of a task.
    pub enum TaskStates {
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

    pub fn update_sp(new_sp: u32) {
        match get_current_tcb() {
            Some(t) => t.sp = new_sp,
            None => {}
        }
    }

    ///
    /// Override current task state with parameter offered
    /// by enum TaskStates
    /// 
    pub fn set_task_state(new_state: TaskStates) {
        match get_current_tcb() {
            Some(t) => t.state = new_state,
            None => {}
        }
    }

    ///
    /// Tasklist is fix located at 0x2000_0200.
    /// TODO: Reserve this area in linker script somehow.
    /// 
    const TCB_START: u32 = 0x2000_0200;
    const TCB_SIZE: u32 = core::mem::size_of::<TCB>() as u32;

    static HEAP_SIZE: AtomicU32 = AtomicU32::new(0);
    pub static CURRENTLY_SLEEPING: AtomicU32 = AtomicU32::new(0);
    pub static CURRENT_TASK: AtomicU32 = AtomicU32::new(0);


    ///
    /// Loads table index of task which has currently the state "SLEEPING",
    /// sets this task into running and returns the last known stackpointer
    /// adress from it.
    /// 
    pub fn get_sleeping() -> u32 {
        let currently_sleeping = CURRENTLY_SLEEPING.load( Ordering::Relaxed) as u32;
        let target_tcb_adress = (currently_sleeping * TCB_SIZE) + TCB_START;
        let tcb = unsafe { &mut *(target_tcb_adress as *mut Option<TCB>) };
        CURRENT_TASK.store(currently_sleeping, Ordering::Relaxed);

        match tcb {
            Some(t) => {
                t.state = TaskStates::RUNNING;
                t.sp
            }
            None => {
                0x00
            }        
        }
    }

    pub fn next_process() -> u32 {
        let current = CURRENT_TASK.fetch_add(1, Ordering::Relaxed) as u32;
        let next = (current + 1) % HEAP_SIZE.load(Ordering::Relaxed);
        let target_tcb_adress = (next * TCB_SIZE) + TCB_START;
        let tcb = unsafe { &mut *(target_tcb_adress as *mut Option<TCB>) };

        CURRENT_TASK.store(next, Ordering::Relaxed);
        let sp_of_next_process: u32;
        match tcb {
            Some(t) => match t.state {
                TaskStates::READY => {
                    sp_of_next_process = t.sp;
                }
                TaskStates::BLOCKED => {
                    sp_of_next_process = next_process();
                }
                TaskStates::TERMINATED  => {
                    sp_of_next_process = next_process();
                }
                _ => {
                    sp_of_next_process = t.sp;
                }
            },
            None => {
                sp_of_next_process = 0x00;
            }
        }
        sp_of_next_process
    }

    fn get_current_tcb<'a>() -> &'a mut Option<TCB> {
        let current = CURRENT_TASK.load(Ordering::Relaxed) as u32;
        let target_tcb_adress = (current * TCB_SIZE) + TCB_START;
        unsafe { &mut *(target_tcb_adress as *mut Option<TCB>) }
    }



    pub fn terminate_task() {
        match get_current_tcb() {
            Some(t) => t.state = TaskStates::TERMINATED,
            None => {}
        }
    }

    pub fn mark_self_as_sleeping() {
        set_task_state(TaskStates::BLOCKED);
        CURRENTLY_SLEEPING.store(CURRENT_TASK.load(Ordering::Relaxed), Ordering::Relaxed);
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
        ///
        /// Points to extern asm instruction. Moves
        /// the current program stack pointer
        /// to cpu register r0
        /// 
        pub fn __write_psp(addr: u32);
        fn __save_process_context();
        fn __load_process_context(addr: u32);
        fn __get_current_psp() -> u32;
    }
    use super::task_control::{get_sleeping, next_process, update_sp};

    pub fn immediate_start(addr: *const u32) {
        unsafe {
            __load_process_context(addr as u32);
        }
    }

    ///
    /// Loads the sleeping task and schedules it instant. 
    /// 
    pub fn priority_schedule() {
        unsafe {
            // loads process stack pointer value into r0,
            // based on this adress registers r4 - r11 gets
            // pushed onto the stack. after finishing this operation,
            // the new value of r0 (it points now to lower adresses 
            // because registers get pushed onto it) gets assigned 
            // to psp.
            __save_process_context();

            // the newly written process stack pointer gets written
            // into the task control block of the process table 
            // for further restoring when needed
            update_sp(__get_current_psp());

            // the saved task's state gets changed from running 
            // to ready, because no other event blocks or terminates
            // the task
            // set_task_state(task_control::TaskStates::READY);

            // the function next_process returns an u32 adress 
            // to the runnable successors task's stackpointer, which is
            // saved in the process table. the parameter gets saved
            // into r0, based from this value the registers r4 - r11
            // gets popped of the stack and written into the cpu's registers. 
            __load_process_context(get_sleeping());
        }
    }

    ///
    /// Loads next ready task in line. 
    /// Schedules tasks in round robin manner.
    /// 
    pub fn context_switch() {
        unsafe {
            // loads process stack pointer value into r0,
            // based on this adress registers r4 - r11 gets
            // pushed onto the stack. after finishing this operation,
            // the new value of r0 (it points now to lower adresses 
            // because registers get pushed onto it) gets assigned 
            // to psp.
            __save_process_context();

            // the newly written process stack pointer gets written
            // into the task control block of the process table 
            // for further restoring when needed
            update_sp(__get_current_psp());

            // the saved task's state gets changed from running 
            // to ready, because no other event blocks or terminates
            // the task
            // set_task_state(task_control::TaskStates::READY);

            // the function next_process returns an u32 adress 
            // to the runnable successors task's stackpointer, which is
            // saved in the process table. the parameter gets saved
            // into r0, based from this value the registers r4 - r11
            // gets popped of the stack and written into the cpu's registers. 
            __load_process_context(next_process());
        }
    }
}
