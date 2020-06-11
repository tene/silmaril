//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate panic_semihosting;

use apa102_spi::Apa102;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use nb::block;
use smart_leds::{
    //hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
    RGB,
    RGB8,
};
use stm32f1xx_hal::{
    gpio::{
        gpioa::{PA5, PA6, PA7},
        gpiob::{PB10, PB11, PB12, PB13, PB14, PB15},
        Alternate, Floating, GpioExt, Input, OpenDrain, PullDown, PushPull,
    },
    pac,
    prelude::*,
    spi::Spi,
    stm32::{I2C2, SPI1},
    timer::Timer,
};

use silmaril::hsv::HSV;

#[derive(Copy, Clone)]
pub struct Demo1 {
    count: u8,
    color: HSV,
    offset: i16,
    stride: u8,
}

impl Demo1 {
    pub fn new(count: u8, color: HSV, offset: i16) -> Self {
        Self {
            count,
            color,
            offset,
            stride: 5,
        }
    }
}

impl Iterator for Demo1 {
    type Item = HSV;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
        if self.stride == 0 {
            self.color.shift_hue(self.offset);
            self.stride = 4;
        } else {
            self.stride -= 1;
        }
        if self.stride % 2 != 0 {
            Some(self.color.shifted_hue(self.offset))
        } else {
            Some(self.color)
        }
    }
}

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(24.mhz())
        .freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(100.hz());

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let pa5 = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let pa6 = gpioa.pa6.into_floating_input(&mut gpioa.crl);
    let pa7 = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let spi_pins = (pa5, pa6, pa7);
    let spi = Spi::spi1(
        dp.SPI1,
        spi_pins,
        &mut afio.mapr,
        apa102_spi::MODE,
        8_000_000.hz(),
        //24_000_000.hz(),
        clocks,
        &mut rcc.apb2,
    );
    let mut lantern = Apa102::new(spi);

    let mut start_color = HSV::new(0, 255, 64);

    // Wait for the timer to trigger an update and change the state of the LED
    let mut buf: [RGB8; 125] = [RGB::new(0, 0, 0); 125];
    loop {
        //led.set_high().unwrap();
        start_color.shift_hue(7);
        let x = Demo1::new(25, start_color, 150);
        let y = x.chain(x).chain(x).chain(x).chain(x);
        for (i, px) in y.enumerate() {
            buf[i] = px.into();
        }
        let _ = lantern.write(buf.iter().cloned());
        block!(timer.wait()).unwrap();
    }
}
