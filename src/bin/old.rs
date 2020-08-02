//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![no_std]
#![no_main]

use apa102_spi::Apa102;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
//use nb::block;
use rtt_target::{rprintln, rtt_init, set_print_channel};
use smart_leds::SmartLedsWrite;
use stm32f4xx_hal::{gpio::GpioExt, prelude::*, spi::Spi, stm32 as pac};

extern crate panic_semihosting;
//use panic_rtt_target as _;

use silmaril::{effect::*, lch_color, Color, Lantern};

#[entry]
fn main() -> ! {
    let channels = rtt_init! {
        up: {
            0: { // channel number
                size: 10240 // buffer size in bytes
                mode: NoBlockSkip // mode (optional, default: NoBlockSkip, see enum ChannelMode)
                name: "Terminal" // name (optional, default: no name)
            }
            1: {
                size: 32
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
    // Get access to the core peripherals from the cortex-m crate
    let _cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    //let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.use_hse(25.mhz()).sysclk(100.mhz()).freeze();

    // Acquire the GPIOC peripheral
    let gpioc = dp.GPIOC.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output();
    // Configure the syst timer to trigger an update every second

    //let mut afio = dp.AFIO.constrain();
    let gpioa = dp.GPIOA.split();

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

    led.set_high().unwrap();

    let mut lantern = Apa102::new(spi);

    // green: 512
    // yellow: 256
    // orange: 128
    // red: 0
    // let _white = Color::new(1.0.into(), 0.0.into(), 0.0.into());
    let _black = lch_color(0.0, 0.0, 0.0);
    //let framerate = 2.hz();
    //let mut timer = Timer::syst(cp.SYST, framerate, clocks);
    //let mut buf: [RGB8; 125] = [RGB::new(0, 0, 0); 125];
    //let mut effect = Demo2::new(start_color, 7, 4);
    //let mut effect = Drops::new(start_color);
    //let mut effect = Solid::new(white, 0);
    //let mut effect = Cloud::new(start_color, 0.01);
    let start_color = lch_color(10.0, 100.0, 300.0);
    let mut rainbow = Rainbow::new(start_color, 2.0, 360.0);
    let dim = Color::new(5.0, 5.0, 305.0);
    let drop = Color::new(0.0, 0.0, 305.0);
    let mut storm = Storm::new(dim, drop, 0.01, 0.05, 0.02, 0.015, 0.8);
    let mut sparks = Sparks::new(0.2, 0.0);
    let effects: [&mut dyn Effect<Lantern>; 3] = [&mut storm, &mut rainbow, &mut sparks];
    let mut model = Lantern::new(_black);
    rprintln!("Starting loop");
    let mut ctr = 0usize;
    let effect_id = 0;
    loop {
        effects[effect_id].tick();
        effects[effect_id].render(&mut model);
        ctr += 1;
        if ctr == 30 {
            rprintln!("30 Ticks Complete");
            //rprintln!("Color: {:?}", effect.color);
            ctr = 0;
        }
        let mut buf = [[0; 3]; 125];
        model.render(&mut buf);
        //rprintln!("Render Complete: {:?}", buf[0]);
        let _ = lantern.write(buf.iter().cloned());
        //rprintln!("Write Complete");
        //block!(timer.wait()).unwrap();
        //rprintln!("Sleep Complete");
    }
}
