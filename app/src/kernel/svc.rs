/// Supervisor Call
///
use crate::{__get_r0, __save_svc_result_to_r5, __sprint, __sprintc, __sreadc, __syscall};

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum SVC {
    SYS_WRITE0(*const u8),
    SYS_WRITEC(*const u8),
    SYS_READC,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum SvcResult {
    None,
    Char(u8),
}

#[no_mangle]
pub extern "C" fn SVCall() {
    let svc = unsafe { &*(__get_r0() as *const SVC) };
    let result: *mut SvcResult;
    match svc {
        SVC::SYS_WRITE0(text) => {
            unsafe { __sprint(*text) }
            result = &mut SvcResult::None
        }
        SVC::SYS_WRITEC(char) => {
            unsafe { __sprintc(*char) }
            result = &mut SvcResult::None
        }
        SVC::SYS_READC => unsafe {
            result = &mut SvcResult::Char(__sreadc());
        },
    }
    unsafe {
        __save_svc_result_to_r5(result);
    };
}

pub fn sprint(text: &str) {
    let tmp_array: &mut [u8; 32] = &mut [0; 32];
    if let Some(last_char) = &mut tmp_array.last() {
        *last_char = &"\0".as_bytes()[0];
    };
    for (index, empty_char) in tmp_array.iter_mut().enumerate() {
        match text.as_bytes().get(index) {
            Some(char) => *empty_char = *char,
            None => {
                *empty_char = "\0".as_bytes()[0];
                break;
            }
        }
    }

    unsafe {
        __syscall(&SVC::SYS_WRITE0(tmp_array.as_ptr()));
    };
}
