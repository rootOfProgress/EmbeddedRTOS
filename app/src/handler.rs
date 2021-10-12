use core::ptr::{read_volatile, write_volatile};

use crate::dp::bus::GPIO;

pub struct LED {
    gpioe: &'static mut GPIO,
}

impl LED {
    pub fn new(gpioe: &'static mut GPIO) -> LED {
        LED { gpioe }
    }

    pub fn on(&mut self, led: u8) {
        if (led <= 15) & (led >= 8) {
            unsafe {
                write_volatile(
                    &mut self.gpioe.moder as *mut u32,
                    read_volatile(&mut self.gpioe.moder) | (0b01 as u32) << (led * 2),
                );
                write_volatile(
                    &mut self.gpioe.otyper as *mut u32,
                    read_volatile(&mut self.gpioe.otyper) & !(1 as u32) << led,
                );
                write_volatile(
                    &mut self.gpioe.odr as *mut u32,
                    read_volatile(&mut self.gpioe.odr) | (0b1 as u32) << led,
                );
            }
        }
    }

    pub fn toggle(&mut self, led: u8) {
        if (led <= 15) & (led >= 8) {
            let odr = unsafe { read_volatile(&mut self.gpioe.odr) };
            if odr & (1 << led) != 0 {
                unsafe {
                    write_volatile(&mut self.gpioe.odr as *mut u32, odr ^ 0b1 << led);
                }
            } else {
                unsafe {
                    write_volatile(&mut self.gpioe.odr as *mut u32, odr | 0b1 << led);
                }
            }
        }
    }
}
