#![no_std]
#![no_main]

use apa102_spi::Apa102;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use panic_probe as _;
use rtic::{app, cyccnt::U32Ext};
use rtt_target::{rprintln, rtt_init, set_print_channel};
use silmaril::{
    effect::*,
    lch_color, Click, InputEvent,
    Knobs::{self, *},
    Lantern, Rotary,
};
use smart_leds::SmartLedsWrite;
use stm32f4xx_hal::{
    gpio::{
        gpioa::{PA0, PA10, PA11, PA5, PA6, PA7, PA9},
        gpiob::{PB12, PB13, PB14, PB5, PB6, PB7},
        gpioc::PC13,
        Alternate, Edge, ExtiPin, GpioExt, Input, Output, PullUp, PushPull, AF5,
    },
    prelude::*,
    pwm,
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
        effect: EffectManager<Lantern>,
        led: PC13<Output<PushPull>>,
        user: PA0<Input<PullUp>>,
        knob1: Rotary<PB13<Input<PullUp>>, PB12<Input<PullUp>>, PB14<Input<PullUp>>>,
        knob2: Rotary<PA10<Input<PullUp>>, PA9<Input<PullUp>>, PA11<Input<PullUp>>>,
        knob3: Rotary<PB6<Input<PullUp>>, PB5<Input<PullUp>>, PB7<Input<PullUp>>>,
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
        /*
        B5  - k3b
        B6  - k3a
        B7  - k3c
        ?8
        A9  - k2b
        A10 - k2a
        A11 - k2c
        B12 - k1b
        B13 - k1a
        B14 - k1c
        ?15
        */

        let mut knob1b = gpiob.pb12.into_pull_up_input();
        let mut knob1a = gpiob.pb13.into_pull_up_input();
        let mut knob1_click = gpiob.pb14.into_pull_up_input();
        let mut knob2b = gpioa.pa9.into_pull_up_input();
        let mut knob2a = gpioa.pa10.into_pull_up_input();
        let mut knob2_click = gpioa.pa11.into_pull_up_input();
        let mut knob3b = gpiob.pb5.into_pull_up_input();
        let mut knob3a = gpiob.pb6.into_pull_up_input();
        let mut knob3_click = gpiob.pb7.into_pull_up_input();
        let mut input_pins: [&mut dyn ExtiPin; 9] = [
            &mut knob1b,
            &mut knob1a,
            &mut knob1_click,
            &mut knob2b,
            &mut knob2a,
            &mut knob2_click,
            &mut knob3b,
            &mut knob3a,
            &mut knob3_click,
        ];
        for p in input_pins.iter_mut() {
            p.make_interrupt_source(&mut dp.SYSCFG);
            p.enable_interrupt(&mut dp.EXTI);
            p.trigger_on_edge(&mut dp.EXTI, Edge::RISING_FALLING);
        }
        let knob1 = Rotary::new(knob1a, knob1b, knob1_click);
        let knob2 = Rotary::new(knob2a, knob2b, knob2_click);
        let knob3 = Rotary::new(knob3a, knob3b, knob3_click);

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

        rprintln!("Silmaril!");

        init::LateResources {
            model,
            leds,
            effect,
            led,
            user,
            knob1,
            knob2,
            knob3,
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

    #[task(binds = EXTI15_10, resources = [knob1, knob2, knob3], spawn = [input], priority = 2)]
    fn knob1(cx: knob1::Context) {
        let events1 = handle_knob(cx.resources.knob1, Knob1);
        let events2 = handle_knob(cx.resources.knob2, Knob2);
        let events3 = handle_knob(cx.resources.knob3, Knob3);
        for event in events1
            .iter()
            .chain(events2.iter())
            .chain(events3.iter())
            .flatten()
        {
            let _ = cx.spawn.input(*event);
        }
    }
    #[task(binds = EXTI9_5, resources = [knob1, knob2, knob3], spawn = [input], priority = 2)]
    fn knob2(cx: knob2::Context) {
        let events1 = handle_knob(cx.resources.knob1, Knob1);
        let events2 = handle_knob(cx.resources.knob2, Knob2);
        let events3 = handle_knob(cx.resources.knob3, Knob3);
        for event in events1
            .iter()
            .chain(events2.iter())
            .chain(events3.iter())
            .flatten()
        {
            let _ = cx.spawn.input(*event);
        }
    }

    #[task(capacity=20, resources = [effect])]
    fn input(cx: input::Context, event: InputEvent) {
        //rprintln!("{:?}", event);
        cx.resources.effect.handle_event(event);
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

fn handle_knob<A, B, C>(knob: &mut Rotary<A, B, C>, kind: Knobs) -> [Option<InputEvent>; 2]
where
    A: InputPin + ExtiPin,
    B: InputPin + ExtiPin,
    C: InputPin + ExtiPin,
    A::Error: core::fmt::Debug,
    B::Error: core::fmt::Debug,
    C::Error: core::fmt::Debug,
{
    let (dir, button) = knob.update();
    let spin = dir.map(|d| InputEvent::Spin(kind, d));
    let click = button.map(|e| match e {
        Click::Press => InputEvent::Press(kind),
        Click::Release => InputEvent::Release(kind),
    });
    [spin, click]
}
