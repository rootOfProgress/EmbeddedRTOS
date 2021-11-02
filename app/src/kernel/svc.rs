/// Supervisor Call
///
use crate::{__get_r0, __sprint, __sprintc, __sreadc, __syscall};

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum SVC {
    SYS_WRITE0(*const u8),
    SYS_WRITEC(*const u8),
    SYS_READC(*const u8),
}

#[no_mangle]
pub extern "C" fn SVCall() {
    let csv = unsafe { &*(__get_r0() as *const SVC) };
    match csv {
        SVC::SYS_WRITE0(text) => unsafe { __sprint(*text) },
        SVC::SYS_WRITEC(char) => unsafe { __sprintc(*char) },
        SVC::SYS_READC(char) => unsafe { __sreadc(*char) },
    }
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

    unsafe { __syscall(&SVC::SYS_WRITE0(tmp_array.as_ptr())) };
}
