use crate::utils::{get_channel_by_name_rgb_color, get_channel_by_name_rgba_u8};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, Rgba, RgbaImage};
use rayon::prelude::*;

pub fn overlay(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: Rgb<u8>,
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

        let (r, g, b) = (
            if lhs.0 < 128 {
                ((lhs.0 as u16 * rhs.0 as u16) / 128) as u8
            } else {
                255 - (((255 - lhs.0 as u16) * (255 - rhs.0 as u16)) / 128) as u8
            },
            if lhs.1 < 128 {
                ((lhs.1 as u16 * rhs.1 as u16) / 128) as u8
            } else {
                255 - (((255 - lhs.1 as u16) * (255 - rhs.1 as u16)) / 128) as u8
            },
            if lhs.2 < 128 {
                ((lhs.2 as u16 * rhs.2 as u16) / 128) as u8
            } else {
                255 - (((255 - lhs.2 as u16) * (255 - rhs.2 as u16)) / 128) as u8
            },
        );

        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

pub fn screen(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: Rgb<u8>,
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

        let (r, g, b) = (
            255 - ((255 - lhs.0) as u16 * (255 - rhs.0) as u16 / 255) as u8,
            255 - ((255 - lhs.1) as u16 * (255 - rhs.1) as u16 / 255) as u8,
            255 - ((255 - lhs.2) as u16 * (255 - rhs.2) as u16 / 255) as u8,
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
    fn test_overlay() {
        let red = load_image("ff0000.png".to_string());
        let control_color = get_color_from_control(red.clone());

        let out = overlay(red.clone(), None, None, Rgb([255, 0, 0]));

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([255, 0, 0]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }

    #[test]
    fn test_screen() {
        let red = load_image("ff0000.png".to_string());
        let control_color = get_color_from_control(red.clone());

        let out = screen(red.clone(), None, None, Rgb([0, 0, 255]));

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([255, 0, 255]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }
}
