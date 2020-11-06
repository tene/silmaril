use crate::{Color, Direction, InputEvent, Knobs, PixelIndexable};
use generic_array::ArrayLength;
use palette::{Limited, Shade};
use rtt_target::rprintln;

pub mod cloud;
pub mod drops;
pub mod flame;
pub mod rainbow;
pub mod solid;
pub mod sparks;
pub mod storm;

pub use cloud::Cloud;
pub use drops::Drops;
pub use flame::Flame;
pub use rainbow::Rainbow;
pub use solid::Solid;
pub use sparks::Sparks;
pub use storm::Storm;

pub trait Effect<T: PixelIndexable> {
    fn tick(&mut self, color: &mut Color);
    fn render(&self, color: Color, model: &mut T);
    fn rotate_cw(&mut self) {}
    fn rotate_ccw(&mut self) {}
    fn click(&mut self) {}
    fn init(&mut self, model: &mut T) {}
    // XXX TODO input / control channels
}

pub enum EffectCycle<T: PixelIndexable>
where
    T::SIZE: ArrayLength<f32>,
{
    Flame(Flame<T>),
    Rainbow(Rainbow<T>),
    Solid(Solid),
    Storm(Storm<T>),
    //Sparks(Sparks<T>),
}

impl<T: PixelIndexable> EffectCycle<T>
where
    T::SIZE: ArrayLength<f32>,
{
    pub fn new() -> Self {
        //Self::Rainbow(Rainbow::default())
        Self::Flame(Flame::default())
    }
    pub fn prev(&mut self) {
        match self {
            EffectCycle::Flame(_) => *self = EffectCycle::Storm(Storm::default()),
            EffectCycle::Rainbow(_) => *self = EffectCycle::Flame(Flame::default()),
            EffectCycle::Solid(_) => *self = EffectCycle::Rainbow(Rainbow::default()),
            EffectCycle::Storm(_) => *self = EffectCycle::Solid(Solid::default()),
            //EffectCycle::Sparks(_) => *self = EffectCycle::Storm(Storm::default()),
        }
    }
    pub fn next(&mut self) {
        match self {
            EffectCycle::Flame(_) => *self = EffectCycle::Rainbow(Rainbow::default()),
            EffectCycle::Rainbow(_) => *self = EffectCycle::Solid(Solid::default()),
            EffectCycle::Solid(_) => *self = EffectCycle::Storm(Storm::default()),
            EffectCycle::Storm(_) => *self = EffectCycle::Flame(Flame::default()),
            //EffectCycle::Sparks(_) => *self = EffectCycle::Rainbow(Rainbow::default()),
        }
    }
    pub fn effect(&self) -> &dyn Effect<T> {
        match self {
            EffectCycle::Flame(e) => e as &dyn Effect<T>,
            EffectCycle::Rainbow(e) => e as &dyn Effect<T>,
            EffectCycle::Solid(e) => e as &dyn Effect<T>,
            EffectCycle::Storm(e) => e as &dyn Effect<T>,
            //EffectCycle::Sparks(e) => e as &dyn Effect<T>,
        }
    }
    pub fn effect_mut(&mut self) -> &mut dyn Effect<T> {
        match self {
            EffectCycle::Flame(e) => e as &mut dyn Effect<T>,
            EffectCycle::Rainbow(e) => e as &mut dyn Effect<T>,
            EffectCycle::Solid(e) => e as &mut dyn Effect<T>,
            EffectCycle::Storm(e) => e as &mut dyn Effect<T>,
            //EffectCycle::Sparks(e) => e as &mut dyn Effect<T>,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            EffectCycle::Flame(_) => "Flame",
            EffectCycle::Rainbow(_) => "Rainbow",
            EffectCycle::Solid(_) => "Solid",
            EffectCycle::Storm(_) => "Storm",
            //EffectCycle::Sparks(_) => "Sparks",
        }
    }
}

impl<T: PixelIndexable> Effect<T> for EffectCycle<T>
where
    T::SIZE: ArrayLength<f32>,
{
    fn rotate_cw(&mut self) {
        self.effect_mut().rotate_cw()
    }

    fn rotate_ccw(&mut self) {
        self.effect_mut().rotate_ccw()
    }

    fn click(&mut self) {
        self.effect_mut().click()
    }

    fn tick(&mut self, color: &mut Color) {
        self.effect_mut().tick(color)
    }

    fn render(&self, color: Color, model: &mut T) {
        self.effect().render(color, model)
    }
}

pub struct EffectManager<T: PixelIndexable>
where
    T::SIZE: ArrayLength<f32>,
{
    pub ec: EffectCycle<T>,
    pub color: Color,
}

impl<T: PixelIndexable> EffectManager<T>
where
    T::SIZE: ArrayLength<f32>,
{
    pub fn default() -> Self {
        let ec = EffectCycle::new();
        let color = Color::new(50.0, 100.0, 300.0);
        Self { ec, color }
    }

    pub fn tick(&mut self) {
        self.ec.tick(&mut self.color)
    }

    pub fn render(&self, model: &mut T) {
        // TODO Here's where we should apply input feedback
        self.ec.render(self.color, model)
    }

    pub fn handle_event(&mut self, event: InputEvent) {
        use Direction::*;
        use InputEvent::*;
        use Knobs::*;
        match event {
            Press(Knob1) => {
                self.ec.next();
                rprintln!("{}", self.ec.name());
            }
            Spin(Knob1, dir) => {
                self.color = self.color.lighten(dir * 0.02).clamp();
                rprintln!("Luma: {}", self.color.l);
            }
            Spin(Knob2, dir) => {
                self.color.chroma += dir * 2.0;
                self.color.clamp_self();
                rprintln!("Chroma: {}", self.color.chroma);
            }
            Spin(Knob3, Clockwise) => {
                self.ec.rotate_cw();
            }
            Spin(Knob3, CounterClockwise) => {
                self.ec.rotate_ccw();
            }
            Press(Knob3) => {
                self.ec.click();
            }
            Release(Knob3) => {}
            _ => {}
        }
    }
}
