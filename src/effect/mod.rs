use crate::{Color, PixelIndexable};
use rtt_target::rprintln;

pub mod cloud;
pub mod drops;
pub mod rainbow;
pub mod solid;
pub mod sparks;
pub mod storm;

pub use cloud::Cloud;
pub use drops::Drops;
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
    //fn init(&mut self, model: &mut T) {}
    // XXX TODO input / control channels
}

pub enum EffectCycle<T: PixelIndexable> {
    Rainbow(Rainbow<T>),
    Solid(Solid),
    Storm(Storm<T>),
    Sparks(Sparks<T>),
}

impl<T: PixelIndexable> EffectCycle<T> {
    pub fn new() -> Self {
        Self::Rainbow(Rainbow::default())
    }
    pub fn prev(&mut self) {
        match self {
            EffectCycle::Rainbow(_) => *self = EffectCycle::Sparks(Sparks::default()),
            EffectCycle::Solid(_) => *self = EffectCycle::Rainbow(Rainbow::default()),
            EffectCycle::Storm(_) => *self = EffectCycle::Solid(Solid::default()),
            EffectCycle::Sparks(_) => *self = EffectCycle::Storm(Storm::default()),
        }
    }
    pub fn next(&mut self) {
        match self {
            EffectCycle::Rainbow(_) => *self = EffectCycle::Solid(Solid::default()),
            EffectCycle::Solid(_) => *self = EffectCycle::Storm(Storm::default()),
            EffectCycle::Storm(_) => *self = EffectCycle::Sparks(Sparks::default()),
            EffectCycle::Sparks(_) => *self = EffectCycle::Rainbow(Rainbow::default()),
        }
    }
    pub fn effect(&self) -> &dyn Effect<T> {
        match self {
            EffectCycle::Rainbow(e) => e as &dyn Effect<T>,
            EffectCycle::Solid(e) => e as &dyn Effect<T>,
            EffectCycle::Storm(e) => e as &dyn Effect<T>,
            EffectCycle::Sparks(e) => e as &dyn Effect<T>,
        }
    }
    pub fn effect_mut(&mut self) -> &mut dyn Effect<T> {
        match self {
            EffectCycle::Rainbow(e) => e as &mut dyn Effect<T>,
            EffectCycle::Solid(e) => e as &mut dyn Effect<T>,
            EffectCycle::Storm(e) => e as &mut dyn Effect<T>,
            EffectCycle::Sparks(e) => e as &mut dyn Effect<T>,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            EffectCycle::Rainbow(_) => "Rainbow",
            EffectCycle::Solid(_) => "Solid",
            EffectCycle::Storm(_) => "Storm",
            EffectCycle::Sparks(_) => "Sparks",
        }
    }
}

impl<T: PixelIndexable> Effect<T> for EffectCycle<T> {
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

pub struct EffectManager<T: PixelIndexable> {
    ec: EffectCycle<T>,
    color: Color,
}

impl<T: PixelIndexable> EffectManager<T> {
    pub fn default() -> Self {
        let ec = EffectCycle::new();
        let color = Color::new(50.0, 100.0, 300.0);
        Self { ec, color }
    }

    pub fn tick(&mut self) {
        self.ec.tick(&mut self.color)
    }

    pub fn render(&self, model: &mut T) {
        self.ec.render(self.color, model)
    }
    // TODO Refactor to single input method
    pub fn rotate_cw(&mut self) {
        self.ec.rotate_cw()
    }

    pub fn rotate_ccw(&mut self) {
        self.ec.rotate_ccw()
    }

    pub fn click(&mut self) {
        self.ec.next();
        rprintln!("{}", self.effect_name());
    }
    pub fn effect_name(&self) -> &'static str {
        self.ec.name()
    }
}
