use std::str::FromStr;

use crate::{calc_luminance, rgb_to_hsv, utils::get_channel_by_name_rgba_u8};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use rayon::prelude::*;

/// Specify whether the filter should replace colors that are INCLUDED in the range or EXCLUDED
/// from the range.
#[derive(Copy, Clone)]
pub enum FilterType {
    Include,
    Exclude,
}

/// Clap FromStr
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

/// What property to filter by? Minimum and maximum values vary by property. For example, hue is
/// 0-360, while red is 0-255.
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

/// Clap FromStr
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

/// A threshold range that the filter will check in between. This is a dedicated struct because
/// for a CLI frontend, I want to minimize String usage after the initial arg parsing.
#[derive(Clone, Copy, Debug)]
pub struct ThresholdRange {
    min: f64,
    max: f64,
}

/// The filter to perform on the image.
pub struct Filter {
    pub filter_type: FilterType,
    pub filter_param: FilterParam,
    pub threshold_ranges: Vec<ThresholdRange>,
}

/// Parse a vector of strings which contain a number into a vector of ThresholdRanges.
pub fn parse_filter_vec(thresholds_str_vec: Vec<String>) -> Vec<ThresholdRange> {
    let mut thresholds: Vec<ThresholdRange> = vec![];

    let mut iter = thresholds_str_vec.iter();
    while let Some(min_str) = iter.next() {
        if let Some(max_str) = iter.next() {
            let min = min_str
                .parse::<f64>()
                .expect("Failed to parse min threshold as f64");
            let max = max_str
                .parse::<f64>()
                .expect("Failed to parse max threshold as f64");

            thresholds.push(ThresholdRange { min, max });
        } else {
            eprintln!(
                "Warning: Threshold range input has an unmatched min value: {}",
                min_str
            );
        }
    }

    thresholds
}

/// Generate the closure which returns a boolean whether the Rgba<u8> satisfies the filter.
fn generate_filter(filter: Filter) -> impl Fn(&Rgba<u8>) -> bool {
    move |pixel| match filter.filter_param {
        FilterParam::Luminance => {
            let luminance = calc_luminance(*pixel);
            filter
                .threshold_ranges
                .iter()
                .any(|range| luminance > range.min && luminance < range.max)
        }
        FilterParam::Red => filter.threshold_ranges.iter().any(|range| {
            let red = pixel.0[0] as f64;
            red > range.min && red < range.max
        }),
        FilterParam::Green => filter.threshold_ranges.iter().any(|range| {
            let green = pixel.0[1] as f64;
            green > range.min && green < range.max
        }),
        FilterParam::Blue => filter.threshold_ranges.iter().any(|range| {
            let blue = pixel.0[2] as f64;
            blue > range.min && blue < range.max
        }),
        FilterParam::Hue => {
            let (h, _, _) = rgb_to_hsv(*pixel);
            filter
                .threshold_ranges
                .iter()
                .any(|range| h > range.min && h < range.max)
        }
        FilterParam::Saturation => {
            let (_, s, _) = rgb_to_hsv(*pixel);
            filter
                .threshold_ranges
                .iter()
                .any(|range| s > range.min && s < range.max)
        }
        FilterParam::Value => {
            let (_, _, v) = rgb_to_hsv(*pixel);
            filter
                .threshold_ranges
                .iter()
                .any(|range| v > range.min && v < range.max)
        }
    }
}

/// Perform the filter operation on the image. lhs will remap the colors before filtering.
pub fn filter(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    filter: Filter,
    replace_with: Rgba<u8>,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let filter_sorter = generate_filter(Filter {
        filter_type: filter.filter_type,
        filter_param: filter.filter_param,
        threshold_ranges: filter.threshold_ranges,
    });

    output.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        // Parse lhs
        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &in_pixel),
            ),
            None => (in_pixel[0], in_pixel[1], in_pixel[2]),
        };

        // Include or exclude
        match filter.filter_type {
            FilterType::Include => {
                // Remaps the lhs here
                if !filter_sorter(&Rgba([lhs.0, lhs.1, lhs.2, 255u8])) {
                    *pixel = replace_with;
                } else {
                    *pixel = in_pixel;
                }
            }

            // Or here
            FilterType::Exclude => {
                if filter_sorter(&Rgba([lhs.0, lhs.1, lhs.2, 255u8])) {
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
