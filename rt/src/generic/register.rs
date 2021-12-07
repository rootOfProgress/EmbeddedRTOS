pub mod adresses {
    pub const TIM2_BASEADRESS: u32 = 0x4000_0000;
    pub const TIM3_BASEADRESS: u32 = 0x4000_0400;


    // manuel page 55
    pub const RCC: u32 = 0x4002_1000;

}

pub mod offsets {
    pub mod rcc {
        pub const RCC_AHBENR: u32 = 0x14;
        pub const RCC_APB2ENR: u32 = 0x18;
        pub const RCC_APB1ENR: u32 = 0x1C;

    }

}