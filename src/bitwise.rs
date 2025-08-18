use crate::utils::{get_channel_by_name_rgb_color, get_channel_by_name_rgba_u8};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, Rgba, RgbaImage};
use rayon::prelude::*;

pub fn or(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: Rgb<u8>,
    negate: bool,
) -> RgbaImage {
    let r = color.0[0];
    let g = color.0[1];
    let b = color.0[2];

    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (r, g, b),
    };

    output.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &in_pixel),
            ),
            None => (in_pixel[0], in_pixel[1], in_pixel[2]),
        };

        let (r, g, b) = match negate {
            true => (!(lhs.0 | rhs.0), !(lhs.1 | rhs.1), !(lhs.2 | rhs.2)),
            false => ((lhs.0 | rhs.0), (lhs.1 | rhs.1), (lhs.2 | rhs.2)),
        };

        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

pub fn and(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: Rgb<u8>,
    negate: bool,
) -> RgbaImage {
    let r = color.0[0];
    let g = color.0[1];
    let b = color.0[2];

    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (r, g, b),
    };

    output.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &in_pixel),
            ),
            None => (in_pixel[0], in_pixel[1], in_pixel[2]),
        };

        let (r, g, b) = match negate {
            true => (!(lhs.0 & rhs.0), !(lhs.1 & rhs.1), !(lhs.2 & rhs.2)),
            false => ((lhs.0 & rhs.0), (lhs.1 & rhs.1), (lhs.2 & rhs.2)),
        };

        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

pub fn xor(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: Rgb<u8>,
    negate: bool,
) -> RgbaImage {
    let r = color.0[0];
    let g = color.0[1];
    let b = color.0[2];

    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (r, g, b),
    };

    output.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &in_pixel),
            ),
            None => (in_pixel[0], in_pixel[1], in_pixel[2]),
        };

        let (r, g, b) = match negate {
            true => (!(lhs.0 ^ rhs.0), !(lhs.1 ^ rhs.1), !(lhs.2 ^ rhs.2)),
            false => ((lhs.0 ^ rhs.0), (lhs.1 ^ rhs.1), (lhs.2 ^ rhs.2)),
        };

        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

pub enum BitshiftDirection {
    LEFT,
    RIGHT,
}

pub fn bitshift(
    img: DynamicImage,
    direction: BitshiftDirection,
    lhs: Option<Vec<String>>,
    bits: u8,
    raw: bool,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    output.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &in_pixel),
            ),
            None => (in_pixel[0], in_pixel[1], in_pixel[2]),
        };
        let (r, g, b, a) = match direction {
            BitshiftDirection::LEFT => {
                if raw {
                    (
                        ((lhs.0 as u16) << bits) as u8,
                        ((lhs.1 as u16) << bits) as u8,
                        ((lhs.2 as u16) << bits) as u8,
                        in_pixel[3],
                    )
                } else {
                    (
                        ((lhs.0 as u16) << bits).min(255) as u8,
                        ((lhs.1 as u16) << bits).min(255) as u8,
                        ((lhs.2 as u16) << bits).min(255) as u8,
                        in_pixel[3],
                    )
                }
            }
            BitshiftDirection::RIGHT => (
                (lhs.0.wrapping_shr(bits.into())),
                (lhs.1.wrapping_shr(bits.into())),
                (lhs.2.wrapping_shr(bits.into())),
                in_pixel[3],
            ),
        };
        *pixel = Rgba([r, g, b, a]);
    });

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Pixel, Rgb};
    use std::env;
    use std::path::PathBuf;

    fn get_file_path(file_name: String) -> PathBuf {
        let mut path = env::current_dir().expect("Failed to get current directory");
        path.push("assets/control-images/");
        path.push(file_name);
        path
    }

    fn load_image(file_name: String) -> DynamicImage {
        let path = get_file_path(file_name);
        let img = image::open(path).expect("Failed to open image.");

        img
    }

    fn get_color_from_control(img: DynamicImage) -> Rgb<u8> {
        let pixel = img.get_pixel(0, 0);
        return pixel.to_rgb();
    }

    #[test]
    fn test_left() {
        let red = load_image("ff0000.png".to_string());
        let control_color = get_color_from_control(red.clone());

        let out = bitshift(
            red.clone(),
            BitshiftDirection::LEFT,
            Some(vec!["r".to_string(), "g".to_string(), "b".to_string()]),
            1,
            false,
        );

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([255, 0, 0]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }

    #[test]
    fn test_right() {
        let red = load_image("ff0000.png".to_string());
        let control_color = get_color_from_control(red.clone());

        let out = bitshift(
            red.clone(),
            BitshiftDirection::RIGHT,
            Some(vec!["r".to_string(), "g".to_string(), "b".to_string()]),
            1,
            false,
        );

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([127, 0, 0]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }

    #[test]
    fn test_or() {
        let red = load_image("ff0000.png".to_string());
        let control_color = get_color_from_control(red.clone());

        let out = or(
            red.clone(),
            Some(vec!["r".to_string(), "g".to_string(), "b".to_string()]),
            None,
            Rgb([0, 0, 255]),
            false,
        );

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([255, 0, 255]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }

    #[test]
    fn test_and() {
        let red = load_image("ff0000.png".to_string());
        let control_color = get_color_from_control(red.clone());

        let out = and(
            red.clone(),
            Some(vec!["r".to_string(), "g".to_string(), "b".to_string()]),
            None,
            Rgb([0, 0, 255]),
            false,
        );

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([0, 0, 0]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }

    #[test]
    fn test_xor() {
        let red = load_image("ff0000.png".to_string());
        let control_color = get_color_from_control(red.clone());

        let out = xor(
            red.clone(),
            Some(vec!["r".to_string(), "g".to_string(), "b".to_string()]),
            None,
            Rgb([0, 0, 255]),
            false,
        );

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([255, 0, 255]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }
}
