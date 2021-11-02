/// Supervisor Call
///
use crate::{__get_r0, __sprint, __sprintc, __sreadc, __syscall};

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum SvcRequest {
    SYS_WRITE0(*const u8),
    _SYS_WRITEC(*const u8),
    SYS_READC,
}

#[repr(C)]
pub enum SvcResult {
    None,
    Char(u8),
}

#[repr(C)]
pub struct SvcOrder {
    request: SvcRequest,
    response: SvcResult,
}

#[no_mangle]
pub fn syscall(request: SvcRequest) -> SvcResult {
    let order = SvcOrder {
        request,
        response: SvcResult::None,
    };

    unsafe {
        __syscall(&order);
    };
    order.response
}

#[no_mangle]
pub extern "C" fn SVCall() {
    let mut order = unsafe { &mut *(__get_r0()) };
    match order.request {
        SvcRequest::SYS_WRITE0(text) => {
            unsafe { __sprint(text) }
            order.response = SvcResult::None;
        }
        SvcRequest::_SYS_WRITEC(char) => {
            unsafe { __sprintc(char) }
            order.response = SvcResult::None;
        }
        SvcRequest::SYS_READC => unsafe {
            order.response = SvcResult::Char(__sreadc());
        },
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

    syscall(SvcRequest::SYS_WRITE0(tmp_array.as_ptr()));
}
