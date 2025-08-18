use std::str::FromStr;

use crate::{calc_luminance, rgb_to_hsv};
use image::{Rgba, RgbaImage};

#[derive(Copy, PartialEq, Clone)]
pub enum Direction {
    Vertical,
    Horizontal,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vertical" => Ok(Direction::Vertical),
            "horizontal" => Ok(Direction::Horizontal),
            "v" => Ok(Direction::Vertical),
            "h" => Ok(Direction::Horizontal),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

#[derive(Copy, PartialEq, Clone)]
pub enum SortBy {
    Luminance,
    Red,
    Green,
    Blue,
    Hue,
    Saturation,
    Value,
}

impl FromStr for SortBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "luminance" => Ok(SortBy::Luminance),
            "red" => Ok(SortBy::Red),
            "green" => Ok(SortBy::Green),
            "blue" => Ok(SortBy::Blue),
            "hue" => Ok(SortBy::Hue),
            "saturation" => Ok(SortBy::Saturation),
            "value" => Ok(SortBy::Value),
            "l" => Ok(SortBy::Luminance),
            "r" => Ok(SortBy::Red),
            "g" => Ok(SortBy::Green),
            "b" => Ok(SortBy::Blue),
            "h" => Ok(SortBy::Hue),
            "s" => Ok(SortBy::Saturation),
            "v" => Ok(SortBy::Value),

            _ => Err(format!("Invalid sort_by factor: {}", s)),
        }
    }
}

fn generate_filter(
    sort_by: SortBy,
    min_threshold: f64,
    max_threshold: f64,
) -> impl Fn(&Rgba<u8>) -> bool {
    move |pixel| match sort_by {
        SortBy::Luminance => {
            let luminance = calc_luminance(*pixel);
            luminance > min_threshold && luminance < max_threshold
        }
        SortBy::Red => pixel.0[0] as f64 > min_threshold && (pixel.0[0] as f64) < max_threshold,
        SortBy::Green => pixel.0[1] as f64 > min_threshold && (pixel.0[1] as f64) < max_threshold,
        SortBy::Blue => pixel.0[2] as f64 > min_threshold && (pixel.0[2] as f64) < max_threshold,
        SortBy::Hue => {
            let (h, _, _) = rgb_to_hsv(*pixel);
            h > min_threshold && h < max_threshold
        }
        SortBy::Saturation => {
            let (_, s, _) = rgb_to_hsv(*pixel);
            s > min_threshold && s < max_threshold
        }
        SortBy::Value => {
            let (_, _, v) = rgb_to_hsv(*pixel);
            v > min_threshold && v < max_threshold
        }
    }
}

fn generate_sorter(sort_by: SortBy) -> impl Fn(&Rgba<u8>, &Rgba<u8>) -> std::cmp::Ordering {
    move |a, b| match sort_by {
        SortBy::Luminance => calc_luminance(*a)
            .partial_cmp(&calc_luminance(*b))
            .unwrap_or(std::cmp::Ordering::Equal),
        SortBy::Red => a.0[0]
            .partial_cmp(&b.0[0])
            .unwrap_or(std::cmp::Ordering::Equal),
        SortBy::Green => a.0[1]
            .partial_cmp(&b.0[1])
            .unwrap_or(std::cmp::Ordering::Equal),
        SortBy::Blue => a.0[2]
            .partial_cmp(&b.0[2])
            .unwrap_or(std::cmp::Ordering::Equal),
        SortBy::Hue => {
            let (h_a, _, _) = rgb_to_hsv(*a);
            let (h_b, _, _) = rgb_to_hsv(*b);
            h_a.partial_cmp(&h_b).unwrap_or(std::cmp::Ordering::Equal)
        }
        SortBy::Saturation => {
            let (_, s_a, _) = rgb_to_hsv(*a);
            let (_, s_b, _) = rgb_to_hsv(*b);
            s_a.partial_cmp(&s_b).unwrap_or(std::cmp::Ordering::Equal)
        }
        SortBy::Value => {
            let (_, _, v_a) = rgb_to_hsv(*a);
            let (_, _, v_b) = rgb_to_hsv(*b);
            v_a.partial_cmp(&v_b).unwrap_or(std::cmp::Ordering::Equal)
        }
    }
}

pub fn sort(
    img: RgbaImage,
    direction: Direction,
    sort_by: SortBy,
    min_threshold: f64,
    max_threshold: f64,
    reversed: bool,
) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut output: RgbaImage = img.clone();

    let filter = generate_filter(sort_by, min_threshold, max_threshold);
    let sorter = generate_sorter(sort_by);

    match direction {
        Direction::Horizontal => {
            for row in 0..height {
                let mut row_pixels: Vec<_> =
                    (0..width).map(|col| *output.get_pixel(col, row)).collect();

                let mut sortable_pixels: Vec<_> = row_pixels
                    .iter()
                    .filter(|&&pixel| filter(&pixel))
                    .cloned()
                    .collect();

                if reversed {
                    sortable_pixels.sort_by(|a, b| sorter(b, a));
                } else {
                    sortable_pixels.sort_by(&sorter);
                }

                let mut sortable_iter = sortable_pixels.into_iter();

                for pixel in row_pixels.iter_mut() {
                    if filter(pixel) {
                        *pixel = sortable_iter.next().unwrap();
                    }
                }

                for (col, pixel) in row_pixels.iter().enumerate() {
                    output.put_pixel(col as u32, row as u32, *pixel);
                }
            }
        }
        Direction::Vertical => {
            for col in 0..width {
                let mut col_pixels: Vec<_> =
                    (0..height).map(|row| *output.get_pixel(col, row)).collect();

                let mut sortable_pixels: Vec<_> = col_pixels
                    .iter()
                    .filter(|&&pixel| filter(&pixel))
                    .cloned()
                    .collect();

                if reversed {
                    sortable_pixels.sort_by(|a, b| sorter(b, a));
                } else {
                    sortable_pixels.sort_by(&sorter);
                }

                let mut sortable_iter = sortable_pixels.into_iter();

                for pixel in col_pixels.iter_mut() {
                    if filter(pixel) {
                        *pixel = sortable_iter.next().unwrap();
                    }
                }

                for (row, pixel) in col_pixels.iter().enumerate() {
                    output.put_pixel(col as u32, row as u32, *pixel);
                }
            }
        }
    }

    output
}
