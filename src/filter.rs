use std::str::FromStr;

use crate::{
    calc_luminance, rgb_to_hsv,
    utils::{get_channel_by_name_rgb_color, get_channel_by_name_rgba_u8},
};
use clap::builder::styling::RgbColor;
use image::{
    imageops::{blur, fast_blur},
    DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage,
};
use rayon::prelude::*;

#[derive(Copy, Clone)]
pub enum FilterType {
    Include,
    Exclude,
}

impl FromStr for FilterType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "include" => Ok(FilterType::Include),
            "exclude" => Ok(FilterType::Exclude),

            _ => Err(format!("Invalid FilterType name: {}", s)),
        }
    }
}

#[derive(Copy, Clone)]
pub enum FilterParam {
    Luminance,
    Red,
    Green,
    Blue,
    Hue,
    Saturation,
    Value,
}

impl FromStr for FilterParam {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "luminance" => Ok(FilterParam::Luminance),
            "red" => Ok(FilterParam::Red),
            "green" => Ok(FilterParam::Green),
            "blue" => Ok(FilterParam::Blue),
            "hue" => Ok(FilterParam::Hue),
            "saturation" => Ok(FilterParam::Saturation),
            "value" => Ok(FilterParam::Value),
            "l" => Ok(FilterParam::Luminance),
            "r" => Ok(FilterParam::Red),
            "g" => Ok(FilterParam::Green),
            "b" => Ok(FilterParam::Blue),
            "h" => Ok(FilterParam::Hue),
            "s" => Ok(FilterParam::Saturation),
            "v" => Ok(FilterParam::Value),

            _ => Err(format!("Invalid FilterParam name: {}", s)),
        }
    }
}

fn generate_filter(
    filter_param: FilterParam,
    min_threshold: f32,
    max_threshold: f32,
) -> impl Fn(&Rgba<u8>) -> bool {
    move |pixel| match filter_param {
        FilterParam::Luminance => {
            let luminance = calc_luminance(*pixel);
            luminance > min_threshold && luminance < max_threshold
        }
        FilterParam::Red => {
            pixel.0[0] as f32 > min_threshold && (pixel.0[0] as f32) < max_threshold
        }
        FilterParam::Green => {
            pixel.0[1] as f32 > min_threshold && (pixel.0[1] as f32) < max_threshold
        }
        FilterParam::Blue => {
            pixel.0[2] as f32 > min_threshold && (pixel.0[2] as f32) < max_threshold
        }
        FilterParam::Hue => {
            let (h, _, _) = rgb_to_hsv(*pixel);
            h > min_threshold && h < max_threshold
        }
        FilterParam::Saturation => {
            let (_, s, _) = rgb_to_hsv(*pixel);
            s > min_threshold && s < max_threshold
        }
        FilterParam::Value => {
            let (_, _, v) = rgb_to_hsv(*pixel);
            v > min_threshold && v < max_threshold
        }
    }
}

pub fn filter(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    min_threshold: f32,
    max_threshold: f32,
    filter_type: FilterType,
    filter_param: FilterParam,
    replace_with: Rgba<u8>,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let filter = generate_filter(filter_param, min_threshold, max_threshold);

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

        match filter_type {
            FilterType::Include => {
                if !filter(&in_pixel) {
                    *pixel = replace_with;
                } else {
                    *pixel = in_pixel;
                }
            }

            FilterType::Exclude => {
                if filter(&in_pixel) {
                    *pixel = replace_with;
                } else {
                    *pixel = in_pixel;
                }
            }
        }
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
}
