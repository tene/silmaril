use palette::{rgb::Srgb, white_point::D65, Component, ConvertFrom, Lch, Pixel};

pub type Color = Lch<D65, f32>;

pub fn lch_to_rgb(lch: Color) -> [u8; 3] {
    let rgb: Srgb<f32> = Srgb::convert_from(lch);
    let [r, g, b]: [f32; 3] = rgb.into_raw();
    //[(r * 256.0) as u8, (g * 256.0) as u8, (b * 256.0) as u8]
    [r.convert(), g.convert(), b.convert()]
}

pub fn lch_color<T: Into<f32>>(l: T, chroma: T, hue: T) -> Color {
    Color::new(l.into(), chroma.into(), hue.into())
}
