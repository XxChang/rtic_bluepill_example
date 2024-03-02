#![no_main]
#![no_std]
#![deny(warnings)]
#![deny(unsafe_code)]
#![feature(type_alias_impl_trait)]

use panic_probe as _;
use rtic::app;
use stm32f1xx_hal as _;
use stm32f1xx_hal::prelude::*;
// use rtic_m
use defmt_rtt as _; // global logger

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use rtic_monotonics::systick::Systick;

    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: stm32f1xx_hal::gpio::gpioc::PC13<stm32f1xx_hal::gpio::Output<stm32f1xx_hal::gpio::PushPull>>,
        state: bool,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();

        let systick_mono_token = rtic_monotonics::create_systick_token!();
        Systick::start(cx.core.SYST, 36_000_000, systick_mono_token);

        defmt::info!("init!");

        let _clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(36.MHz()).pclk1(36.MHz()).freeze(&mut flash.acr);

        let mut gpioc = cx.device.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        led.set_high();

        blink::spawn().ok();

        (Shared {}, Local {
            led,
            state: false,
        })
    }

    #[task(local = [led, state])]
    async fn blink(cx: blink::Context) {
        loop {
            defmt::info!("blink");
            if *cx.local.state {
                cx.local.led.set_high();
                *cx.local.state = false;
            } else {
                cx.local.led.set_low();
                *cx.local.state = true;
            }
            Systick::delay(1000.millis()).await;
        }
    }
}

