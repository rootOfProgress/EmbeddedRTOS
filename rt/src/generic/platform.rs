pub mod adresses {
    pub const TIM2_BASEADRESS: u32 = 0x4000_0000;
    pub const TIM3_BASEADRESS: u32 = 0x4000_0400;
    pub const USART1_BASEADRESS: u32 = 0x4001_3800;


    // manuel page 55
    pub const RCC: u32 = 0x4002_1000;

}

pub mod offsets {
    pub mod rcc {
        pub const RCC_APB1RSTR: u32 = 0x10;
        pub const RCC_AHBENR: u32 = 0x14;
        pub const RCC_APB2ENR: u32 = 0x18;
        pub const RCC_APB1ENR: u32 = 0x1C;
    }
    pub mod usart1 {
        pub const TDR: u32 = 0x28;
        pub const ISR: u32 = 0x1C;
    }
    pub mod tim {
        pub const DIER: u32 = 0x0C;
        pub const EGR: u32 = 0x14;
        pub const CNT: u32 = 0x24;
        pub const PSC: u32 = 0x28;
        pub const CCR1: u32 = 0x34;

    }
}

pub mod bitfields {
    pub mod rcc {
        pub const IOPAEN: u32 = 17;
        pub const IOPEEN: u32 = 21;
    } 
    pub mod usart1 {
        pub const USART1EN: u32 = 14;
    }
}