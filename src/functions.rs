use clap::builder::styling::RgbColor;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

use crate::utils::{get_channel_by_name_rgb_color, get_channel_by_name_rgba_u8};

pub fn average(
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

    output.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &in_pixel),
            ),
            None => (in_pixel[0], in_pixel[1], in_pixel[2]),
        };

        let (r, g, b) = (
            (lhs.0 + rhs.0) / 2,
            (lhs.1 + rhs.1) / 2,
            (lhs.2 + rhs.2) / 2,
        );

        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

pub fn bloom(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
    intensity: f32,
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

    output.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &in_pixel),
            ),
            None => (in_pixel[0], in_pixel[1], in_pixel[2]),
        };

        let (r, g, b) = (
            ((lhs.0 as f32) + (lhs.0 as f32 * rhs.0 as f32 / 255.0) * intensity).min(255.0) as u8,
            ((lhs.1 as f32) + (lhs.1 as f32 * rhs.1 as f32 / 255.0) * intensity).min(255.0) as u8,
            ((lhs.2 as f32) + (lhs.2 as f32 * rhs.2 as f32 / 255.0) * intensity).min(255.0) as u8,
        );

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
    fn test_average() {
        let red = load_image("ff0000.png".to_string());
        let control_color = get_color_from_control(red.clone());

        let out = average(red.clone(), None, None, RgbColor(0, 0, 255));

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([127, 0, 127]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }

    #[test]
    fn test_bloom() {
        let red = load_image("ff0000.png".to_string());
        let control_color = get_color_from_control(red.clone());

        let out = bloom(red.clone(), None, None, RgbColor(0, 0, 255), 1.0);

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([255, 0, 0]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }
}
