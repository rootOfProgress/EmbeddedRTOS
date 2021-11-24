pub mod memory_handler {
    use core::ptr;

    pub fn write(dest: u32, content: u32) {
        unsafe {
            ptr::write(dest as *mut u32, content);
        }
    }
    pub fn read(dest: u32) -> u32 {
        unsafe { ptr::read_volatile(dest as *const u32) }
    }
}
