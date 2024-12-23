use crate::utils::{get_channel_by_name_rgb_color, get_channel_by_name_rgba_u8};
use clap::builder::styling::RgbColor;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use rayon::prelude::*;

pub fn add(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.g(), color.b()),
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

        let r = lhs.0 + rhs.0;
        let g = lhs.1 + rhs.1;
        let b = lhs.2 + rhs.2;
        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

pub fn sub(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
    raw: bool,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.g(), color.b()),
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

        let (r, g, b) = match raw {
            true => {
                let r = lhs.0 - rhs.0;
                let g = lhs.1 - rhs.1;
                let b = lhs.2 - rhs.2;

                (r, g, b)
            }
            false => {
                let r = (lhs.0 as i32 - rhs.0 as i32).abs() as u8;
                let g = (lhs.1 as i32 - rhs.1 as i32).abs() as u8;
                let b = (lhs.2 as i32 - rhs.2 as i32).abs() as u8;

                (r, g, b)
            }
        };

        let a = in_pixel[3];
        *pixel = Rgba([r, g, b, a]);
    });

    output
}

pub fn mult(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.g(), color.b()),
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

        let r = lhs.0.wrapping_mul(rhs.0);
        let g = lhs.1.wrapping_mul(rhs.1);
        let b = lhs.2.wrapping_mul(rhs.2);
        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

pub fn pow(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.g(), color.b()),
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

        let r = lhs.0.wrapping_pow(rhs.0 as u32) as u8;
        let g = lhs.1.wrapping_pow(rhs.1 as u32) as u8;
        let b = lhs.2.wrapping_pow(rhs.2 as u32) as u8;

        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

pub fn div(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.g(), color.b()),
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

        let r = lhs.0 / rhs.0.max(1);
        let g = lhs.1 / rhs.1.max(1);
        let b = lhs.2 / rhs.2.max(1);
        let a = in_pixel[3];

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
    fn test_add() {
        let red = load_image("ff0000.png".to_string());
        let control_color = get_color_from_control(red.clone());

        let out = add(red.clone(), None, None, RgbColor(0, 0, 255));

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([255, 0, 255]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }

    #[test]
    fn test_sub() {
        let red = load_image("ff0000.png".to_string());
        let control_color = get_color_from_control(red.clone());

        let out = sub(red.clone(), None, None, RgbColor(0, 0, 255), false);

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([255, 0, 255]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }

    #[test]
    fn test_mult() {
        let red = load_image("ff0000.png".to_string());
        let control_color = get_color_from_control(red.clone());

        let out = mult(red.clone(), None, None, RgbColor(0, 0, 255));

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([0, 0, 0]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }

    #[test]
    fn test_div() {
        let red = load_image("ff0000.png".to_string());
        let control_color = get_color_from_control(red.clone());

        let out = div(red.clone(), None, None, RgbColor(0, 0, 255));

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([255, 0, 0]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }
}
