use crate::utils::{get_channel_by_name_rgb_color, get_channel_by_name_rgba_u8};
use clap::builder::styling::RgbColor;
use image::{imageops::fast_blur, DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use rayon::prelude::*;

pub fn greyscale(img: DynamicImage) -> RgbaImage {
    return Into::into(img.grayscale());
}

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
    intensity: f64,
    blur_radius: f64,
    min_threshold: u8,
    max_threshold: Option<u8>,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let rgba_img = img.to_rgba8();

    let mut light_mask: RgbaImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in rgba_img.enumerate_pixels() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        //  = 0.2126 R + 0.7152 G + 0.0722 B
        let luminance = 0.2126 * (r as f64) + 0.7152 * (g as f64) + 0.0722 * (b as f64);

        match max_threshold {
            Some(threshold) => {
                if luminance > min_threshold as f64 && luminance < threshold as f64 {
                    light_mask.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
                } else {
                    light_mask.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                }
            }
            None => {
                if luminance > min_threshold as f64 {
                    light_mask.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
                } else {
                    light_mask.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                }
            }
        }
    }

    let blurred_light = fast_blur(&light_mask, blur_radius as f32);

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in rgba_img.enumerate_pixels() {
        let blurred_pixel = blurred_light.get_pixel(x, y);

        // Blend the blurred light with the original image
        let (r, g, b) = (
            ((pixel[0] as f64) + (blurred_pixel[0] as f64 * intensity)).min(255.0) as u8,
            ((pixel[1] as f64) + (blurred_pixel[1] as f64 * intensity)).min(255.0) as u8,
            ((pixel[2] as f64) + (blurred_pixel[2] as f64 * intensity)).min(255.0) as u8,
        );

        let a = pixel[3];

        output.put_pixel(x, y, Rgba([r, g, b, a]));
    }

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

        let out = bloom(red.clone(), 1.0, 1.0, 0, Some(255));

        println!(
            "{:?} == {:?}",
            control_color,
            out.get_pixel(0, 0).to_rgb().0
        );

        const EXPECTED: Rgb<u8> = Rgb([255, 0, 0]);

        assert_eq!(out.get_pixel(0, 0).to_rgb(), EXPECTED)
    }
}
