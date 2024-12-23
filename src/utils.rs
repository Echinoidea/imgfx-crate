use clap::builder::styling::RgbColor;
use image::Rgba;

pub fn get_channel_by_name_rgb_color(name: &str, color: &RgbColor) -> u8 {
    match name {
        "r" => color.r(),
        "g" => color.g(),
        "b" => color.b(),
        _ => 0,
    }
}

pub fn get_channel_by_name_rgba_u8(name: &str, color: &Rgba<u8>) -> u8 {
    match name {
        "r" => color[0],
        "g" => color[1],
        "b" => color[2],
        _ => 0,
    }
}

pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.starts_with('#') && hex.len() == 7 {
        let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
        Some((r, g, b))
    } else if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some((r, g, b))
    } else {
        None
    }
}

pub fn rgb_to_hsv(rgb: Rgba<u8>) -> (f32, f32, f32) {
    let r = rgb.0[0] as f32 / 255.0;
    let g = rgb.0[1] as f32 / 255.0;
    let b = rgb.0[2] as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / delta % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };

    let s = if max == 0.0 { 0.0 } else { delta / max };

    let v = max;

    (h, s, v)
}

pub fn calc_luminance(color: Rgba<u8>) -> f32 {
    let luminance =
        0.2126 * (color[0] as f32) + 0.7152 * (color[1] as f32) + 0.0722 * (color[2] as f32);
    luminance
}
