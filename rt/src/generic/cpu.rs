pub mod system {
    pub const CLOCK: u32 = 8_000_000;
}

pub mod c_adresses {
    // page 244
    pub const SCB: u32 = 0xE000_ED00;
    pub const STK: u32 = 0xE000_E010;
}

pub mod c_offsets {
    pub mod scb {
        pub const ICSR: u32 = 0x04;
    }
    pub mod stk {
        pub const CTRL: u32 = 0x00;
        pub const LOAD: u32 = 0x04;
        pub const VAL: u32 = 0x08;
        pub const CALIB: u32 = 0x0C;
    }
}

pub mod c_bitfields {
    pub mod icsr {
        pub const PENDSVSET: u32 = 28;
    }
    pub mod stk {
        pub const ENABLE: u32 = 0b1;
    }
}