use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use fixed::{traits::LossyInto, types::I16F16};
use num_traits::{Num, NumCast, One, ToPrimitive, Zero};
use palette::{float::Float, rgb::Srgb, white_point::D65, Component, ConvertFrom, Lch, Pixel};

// TODO Fixed = I16F16 to deal with negatives
pub type Fixed = I16F16;

pub type Color = Lch<D65, Unit>;

pub fn lch_to_rgb(lch: Color) -> [u8; 3] {
    let rgb: Srgb<Unit> = Srgb::convert_from(lch);
    let [r, g, b]: [Unit; 3] = rgb.into_raw();
    [r.as_u8(), g.as_u8(), b.as_u8()]
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Unit(Fixed);

impl Unit {
    pub fn into_f32(self) -> f32 {
        self.0.lossy_into()
    }
    pub fn from_f32(n: f32) -> Self {
        Self(Fixed::from_num(n))
    }
    pub fn as_u8(self) -> u8 {
        let x = I16F16::from_num(self.0);
        x.wrapping_mul_int(256).to_num()
    }
    pub fn wrapping_add(self, rhs: Self) -> Self {
        self.0.wrapping_add(rhs.0).into()
    }
    pub fn saturating_add(self, rhs: Self) -> Self {
        self.0.saturating_add(rhs.0).into()
    }
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        self.0.wrapping_sub(rhs.0).into()
    }
    pub fn saturating_sub(self, rhs: Self) -> Self {
        self.0.saturating_sub(rhs.0).into()
    }
    pub fn wrapping_mul_int(self, rhs: i32) -> Self {
        self.0.wrapping_mul_int(rhs).into()
    }
}

fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

fn cast<T: NumCast, P: ToPrimitive>(prim: P) -> T {
    NumCast::from(prim).unwrap()
}

impl From<Fixed> for Unit {
    fn from(val: Fixed) -> Self {
        Self(val)
    }
}

impl From<f32> for Unit {
    fn from(val: f32) -> Self {
        Self(Fixed::from_num(val))
    }
}

impl Component for Unit {
    const LIMITED: bool = true;
    fn max_intensity() -> Self {
        Self(Fixed::from_num(1))
    }
    fn convert<T: Component>(&self) -> T {
        let scaled = *self * cast(T::max_intensity());

        if T::LIMITED {
            cast(clamp(
                Float::round(scaled),
                0.0.into(),
                cast(T::max_intensity()),
            ))
        } else {
            cast(scaled)
        }
    }
}

impl Float for Unit {
    fn nan() -> Self {
        Self::zero()
    }
    fn infinity() -> Self {
        Self::max_value()
    }
    fn neg_infinity() -> Self {
        Self::min_value()
    }
    fn neg_zero() -> Self {
        Self::zero()
    }
    fn min_value() -> Self {
        Fixed::MIN.into()
    }
    fn min_positive_value() -> Self {
        Fixed::from_bits(1).into()
    }
    fn max_value() -> Self {
        Fixed::MAX.into()
    }
    fn is_nan(self) -> bool {
        false
    }
    fn is_infinite(self) -> bool {
        false
    }
    fn is_finite(self) -> bool {
        true
    }
    fn is_normal(self) -> bool {
        self.0 != 0
    }
    fn classify(self) -> core::num::FpCategory {
        use core::num::FpCategory;
        if self.0 == 0 {
            FpCategory::Zero
        } else {
            FpCategory::Normal
        }
    }
    fn floor(self) -> Self {
        self.0.floor().into()
    }
    fn ceil(self) -> Self {
        self.0.ceil().into()
    }
    fn round(self) -> Self {
        self.0.round().into()
    }
    fn trunc(self) -> Self {
        self.0.int().into()
    }
    fn fract(self) -> Self {
        self.0.frac().into()
    }
    fn abs(self) -> Self {
        self.0.abs().into()
    }
    fn signum(self) -> Self {
        match self.cmp(&Self::zero()) {
            core::cmp::Ordering::Less => Self::one().neg(),
            core::cmp::Ordering::Equal => Self::zero(),
            core::cmp::Ordering::Greater => Self::one(),
        }
    }
    fn is_sign_positive(self) -> bool {
        self.0 > 0
    }
    fn is_sign_negative(self) -> bool {
        self.0 < 0
    }
    fn mul_add(self, a: Self, b: Self) -> Self {
        self.0.saturating_mul(a.0).saturating_add(b.0).into()
    }
    fn recip(self) -> Self {
        Fixed::from_num(1).saturating_div(self.0).into()
    }
    fn powi(self, n: i32) -> Self {
        self.into_f32().powi(n).into()
    }
    fn powf(self, n: Self) -> Self {
        self.into_f32().powf(n.into_f32()).into()
    }
    fn sqrt(self) -> Self {
        self.into_f32().sqrt().into()
    }
    fn exp(self) -> Self {
        self.into_f32().exp().into()
    }
    fn exp2(self) -> Self {
        self.into_f32().exp2().into()
    }
    fn ln(self) -> Self {
        self.into_f32().ln().into()
    }
    fn log(self, base: Self) -> Self {
        self.into_f32().log(base.into_f32()).into()
    }
    fn log2(self) -> Self {
        self.into_f32().log2().into()
    }
    fn log10(self) -> Self {
        self.into_f32().log10().into()
    }
    fn max(self, other: Self) -> Self {
        self.0.max(other.0).into()
    }
    fn min(self, other: Self) -> Self {
        self.0.min(other.0).into()
    }
    fn abs_sub(self, other: Self) -> Self {
        self.into_f32().abs_sub(other.into_f32()).into()
    }
    fn cbrt(self) -> Self {
        self.into_f32().cbrt().into()
    }
    fn hypot(self, other: Self) -> Self {
        self.into_f32().hypot(other.into_f32()).into()
    }
    fn sin(self) -> Self {
        self.into_f32().sin().into()
    }
    fn cos(self) -> Self {
        self.into_f32().cos().into()
    }
    fn tan(self) -> Self {
        self.into_f32().tan().into()
    }
    fn asin(self) -> Self {
        self.into_f32().asin().into()
    }
    fn acos(self) -> Self {
        self.into_f32().acos().into()
    }
    fn atan(self) -> Self {
        self.into_f32().atan().into()
    }
    fn atan2(self, other: Self) -> Self {
        self.into_f32().atan2(other.into_f32()).into()
    }
    fn sin_cos(self) -> (Self, Self) {
        let (s, c): (f32, f32) = self.into_f32().sin_cos();
        (s.into(), c.into())
    }
    fn exp_m1(self) -> Self {
        self.into_f32().exp_m1().into()
    }
    fn ln_1p(self) -> Self {
        self.into_f32().ln_1p().into()
    }
    fn sinh(self) -> Self {
        self.into_f32().sinh().into()
    }
    fn cosh(self) -> Self {
        self.into_f32().cosh().into()
    }
    fn tanh(self) -> Self {
        self.into_f32().tanh().into()
    }
    fn asinh(self) -> Self {
        self.into_f32().asinh().into()
    }
    fn acosh(self) -> Self {
        self.into_f32().acosh().into()
    }
    fn atanh(self) -> Self {
        self.into_f32().atanh().into()
    }
    fn integer_decode(self) -> (u64, i16, i8) {
        self.into_f32().integer_decode()
    }
}

impl NumCast for Unit {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        n.to_f32().map(Self::from_f32)
    }
}

impl ToPrimitive for Unit {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }
    fn to_u64(&self) -> Option<u64> {
        todo!()
    }
    // XXX TODO Implement these
    fn to_u8(&self) -> Option<u8> {
        todo!()
    }
    fn to_u16(&self) -> Option<u16> {
        todo!()
    }
    fn to_f32(&self) -> Option<f32> {
        Some(self.0.lossy_into())
    }
    fn to_f64(&self) -> Option<f64> {
        Some(self.0.lossy_into())
    }
}

impl Num for Unit {
    type FromStrRadixErr = fixed::ParseFixedError;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let rv = match radix {
            2 => Fixed::from_str_binary(str),
            8 => Fixed::from_str_octal(str),
            16 => Fixed::from_str_hex(str),
            _ => Fixed::saturating_from_str(str),
        };
        rv.map(Self)
    }
}

impl Zero for Unit {
    fn zero() -> Self {
        Fixed::MIN.into()
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl One for Unit {
    fn one() -> Self {
        Fixed::MAX.into()
    }
}

impl Add for Unit {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}
impl Mul for Unit {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.0.mul(rhs.0).into()
    }
}
impl Sub for Unit {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}
impl Div for Unit {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.0.div(rhs.0).into()
    }
}
impl Rem for Unit {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        self.0.rem(rhs.0).into()
    }
}
impl Neg for Unit {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::zero()
    }
}

fn _test() {
    let mut _c = Color::new::<Unit>(0.5.into(), 0.5.into(), 0.5.into());
}

/*
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Color {
    hue: Unit,
    sat: Unit,
    lit: Unit,
}
*/
