#![no_std]
#![no_main]
#![allow(unused_imports)]

use panic_itm;
use cortex_m::Peripherals as cp_;
use cortex_m_rt::entry;
use stm32l4xx_hal::{delay::Delay, prelude::*, stm32::Peripherals as dp_};



#[entry]
fn main() -> ! {
    // using stm32l4xx_hal
    // https://github.com/stm32-rs/stm32l4xx-hal/blob/master/examples/blinky.rs

    // cortex m peripherals
    let cp = cp_::take().unwrap();
    // device-specific peripherals
    let dp = dp_::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr
        //.hclk(8.MHz())
        .sysclk(64.MHz())
        .pclk1(32.MHz())
        .freeze(&mut flash.acr, &mut pwr)
        ;

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut timer = Delay::new(cp.SYST, clocks);

    loop {
        timer.delay_ms(1000_u32);
        led.set_high();
        timer.delay_ms(1000_u32);
        led.set_low();
    }
}
