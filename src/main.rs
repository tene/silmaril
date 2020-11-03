#![no_std]
#![no_main]

extern crate panic_semihosting;

use apa102_spi::Apa102;
use embedded_hal::digital::v2::OutputPin;
use rtic::{app, cyccnt::U32Ext};
use rtt_target::{rprintln, rtt_init, set_print_channel};
use silmaril::{effect::*, lch_color, Direction, Lantern, Rotary};
use smart_leds::SmartLedsWrite;
use stm32f4xx_hal::{
    gpio::{
        gpioa::{PA0, PA5, PA6, PA7},
        gpiob::{PB12, PB13, PB14},
        gpioc::PC13,
        Alternate, Edge, ExtiPin, GpioExt, Input, Output, PullUp, PushPull, AF5,
    },
    prelude::*,
    pwm,
    spi::Spi,
    stm32 as pac,
};

type Knob = Rotary<PB12<Input<PullUp>>, PB13<Input<PullUp>>>;

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
        effect: EffectManager<Lantern>,
        led: PC13<Output<PushPull>>,
        user: PA0<Input<PullUp>>,
        knob: Knob,
        knob_click: PB14<Input<PullUp>>,
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

        // Required for interrupts on GPIO ports
        dp.RCC.apb2enr.write(|w| w.syscfgen().enabled());

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

        let gpiob = dp.GPIOB.split();

        let fan_pin = gpiob.pb4.into_alternate_af2();
        let mut fan = pwm::tim3(dp.TIM3, fan_pin, clocks, 25u32.khz());
        let fan_max_duty = fan.get_max_duty();
        let _ = fan.set_duty(fan_max_duty / 3);
        let _ = fan.enable();

        //let mut knob1 = gpioa.pa2.into_pull_up_input();
        let mut knob1 = gpiob.pb12.into_pull_up_input();
        knob1.make_interrupt_source(&mut dp.SYSCFG);
        knob1.enable_interrupt(&mut dp.EXTI);
        knob1.trigger_on_edge(&mut dp.EXTI, Edge::RISING_FALLING);
        //let mut knob2 = gpioa.pa3.into_pull_up_input();
        let mut knob2 = gpiob.pb13.into_pull_up_input();
        knob2.make_interrupt_source(&mut dp.SYSCFG);
        knob2.enable_interrupt(&mut dp.EXTI);
        knob2.trigger_on_edge(&mut dp.EXTI, Edge::RISING_FALLING);
        //let mut knob_click = gpioa.pa4.into_pull_up_input();
        let mut knob_click = gpiob.pb14.into_pull_up_input();
        knob_click.make_interrupt_source(&mut dp.SYSCFG);
        knob_click.enable_interrupt(&mut dp.EXTI);
        knob_click.trigger_on_edge(&mut dp.EXTI, Edge::RISING_FALLING);
        let knob = Rotary::new(knob1, knob2);

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
        let effect = EffectManager::default();
        let black = lch_color(0.0, 0.0, 0.0);
        let model = Lantern::new(black);

        cx.schedule.tick(cx.start + PERIOD.cycles()).unwrap();

        init::LateResources {
            model,
            leds,
            effect,
            led,
            user,
            knob,
            knob_click,
        }
    }

    #[task(resources = [model, leds, effect], schedule = [tick])]
    fn tick(cx: tick::Context) {
        cx.resources.effect.tick();
        let model: &mut Lantern = cx.resources.model;
        cx.resources.effect.render(model);
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

    #[task(binds = EXTI15_10, resources = [knob, knob_click], spawn = [input], priority = 2)]
    fn knob1(cx: knob1::Context) {
        //cx.resources.effect.rotate_cw();
        if let Some(dir) = cx.resources.knob.update() {
            use Direction::*;
            match dir {
                Clockwise => {
                    let _ = cx.spawn.input(InputEvent::Clockwise);
                }
                CounterClockwise => {
                    let _ = cx.spawn.input(InputEvent::CounterClockwise);
                }
            }
        }
        let knob_click: &mut _ = cx.resources.knob_click;
        knob_click.clear_interrupt_pending_bit();
        unsafe {
            static mut CLICKED: bool = true;
            match (knob_click.is_high().unwrap(), CLICKED) {
                (true, false) => {
                    CLICKED = true;
                    let _ = cx.spawn.input(InputEvent::Click);
                }
                (false, true) => {
                    CLICKED = false;
                }
                _ => {}
            }
        }
        rprintln!("Knob");
    }

    #[task(capacity=10, resources = [effect])]
    fn input(cx: input::Context, input: InputEvent) {
        match input {
            InputEvent::Clockwise => cx.resources.effect.rotate_cw(),
            InputEvent::CounterClockwise => cx.resources.effect.rotate_ccw(),
            InputEvent::Click => cx.resources.effect.click(),
        }
    }

    // Work around https://github.com/probe-rs/probe-rs/issues/300
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            core::sync::atomic::spin_loop_hint();
        }
    }

    extern "C" {
        fn USART1();
    }
};

enum InputEvent {
    Clockwise,
    CounterClockwise,
    Click,
}
