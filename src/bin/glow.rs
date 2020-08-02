#![no_std]
#![no_main]

extern crate panic_semihosting;

use apa102_spi::Apa102;
use embedded_hal::digital::v2::OutputPin;
use rtic::{app, cyccnt::U32Ext};
use rtt_target::{rprintln, rtt_init, set_print_channel};
use silmaril::{effect::*, lch_color, Color, Lantern};
use smart_leds::SmartLedsWrite;
use stm32f4xx_hal::{
    gpio::{
        gpioa::{PA0, PA5, PA6, PA7},
        gpioc::PC13,
        Alternate, Edge, ExtiPin, GpioExt, Input, Output, PullUp, PushPull, AF5,
    },
    prelude::*,
    spi::Spi,
    stm32 as pac,
};

const PERIOD: u32 = 10_000_000;

#[app(device = stm32f4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        model: Lantern,
        leds: Apa102<
            Spi<
                pac::SPI1,
                (
                    PA5<Alternate<AF5>>,
                    PA6<Alternate<AF5>>,
                    PA7<Alternate<AF5>>,
                ),
            >,
        >,
        storm: Storm<Lantern>,
        led: PC13<Output<PushPull>>,
        user: PA0<Input<PullUp>>,
    }
    #[init(schedule = [tick])]
    fn init(cx: init::Context) -> init::LateResources {
        //Enable cycle counter
        let mut core = cx.core;
        core.DWT.enable_cycle_counter();

        let channels = rtt_init! {
            up: {
                0: { // channel number
                    size: 10240 // buffer size in bytes
                    mode: NoBlockSkip // mode (optional, default: NoBlockSkip, see enum ChannelMode)
                    name: "Terminal" // name (optional, default: no name)
                }
            }
            down: {
                0: {
                    size: 16
                    name: "Terminal"
                }
            }
        };
        set_print_channel(channels.up.0);

        let mut dp = cx.device;
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(25.mhz()).sysclk(100.mhz()).freeze();
        let gpioc = dp.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output();
        let _ = led.set_high();

        let gpioa = dp.GPIOA.split();

        let mut user = gpioa.pa0.into_pull_up_input();
        user.make_interrupt_source(&mut dp.SYSCFG);
        user.enable_interrupt(&mut dp.EXTI);
        user.trigger_on_edge(&mut dp.EXTI, Edge::RISING);

        let pa5 = gpioa.pa5.into_alternate_af5();
        let pa6 = gpioa.pa6.into_alternate_af5();
        let pa7 = gpioa.pa7.into_alternate_af5();

        let spi_pins = (pa5, pa6, pa7);
        let spi = Spi::spi1(
            dp.SPI1,
            spi_pins,
            apa102_spi::MODE,
            1_000_000.hz(),
            //24_000_000.hz(),
            clocks,
        );

        let leds = Apa102::new(spi);
        let dim = Color::new(5.0, 5.0, 305.0);
        let drop = Color::new(0.0, 0.0, 305.0);
        let storm = Storm::new(dim, drop, 0.01, 0.05, 0.02, 0.015, 0.8);
        let black = lch_color(0.0, 0.0, 0.0);
        let model = Lantern::new(black);

        cx.schedule.tick(cx.start + PERIOD.cycles()).unwrap();

        init::LateResources {
            model,
            leds,
            storm,
            led,
            user,
        }
    }

    #[task(resources = [model, leds, storm], schedule = [tick])]
    fn tick(cx: tick::Context) {
        cx.resources.storm.tick();
        let model: &mut Lantern = cx.resources.model;
        cx.resources.storm.render(model);
        let mut buf = [[0; 3]; 125];
        model.render(&mut buf);
        let _ = cx.resources.leds.write(buf.iter().cloned());
        cx.schedule.tick(cx.scheduled + PERIOD.cycles()).unwrap();
    }

    #[task(binds = EXTI0, resources = [user, led])]
    fn user(cx: user::Context) {
        rprintln!("User button pushed");
        let _ = cx.resources.led.toggle();
        cx.resources.user.clear_interrupt_pending_bit();
    }

    extern "C" {
        fn USART1();
    }
};
