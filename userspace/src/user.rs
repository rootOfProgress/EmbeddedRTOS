#![forbid(unsafe_code)]

pub mod user {
    use rt::sys::call_api;

    fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 1,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }
    // use 
    pub fn context3() {
        loop {
            fibonacci(21);
            call_api::println("wake up task0!\n\r\0");
        }
    }
    
    pub fn context2() {
        loop {
            fibonacci(22);
            call_api::println("done task1!\n\r\0");   
        }
    }

    pub fn context1() {
        loop {
            fibonacci(22);
            call_api::println("done task2!\n\r\0");   
        }
    }

    pub fn context0() {
        loop {
            call_api::sleep(500);
            call_api::println("wake up!\n\r\0");   
        }
    }
}