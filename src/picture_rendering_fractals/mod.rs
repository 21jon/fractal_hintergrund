use image::{ImageBuffer, ImageFormat};
use nalgebra::Complex;
use nalgebra::Vector3;
use num_cpus;
use palette::gradient;
use palette::Blend;
use palette::{LinSrgb, Pixel, Srgb};
use std::sync::Arc;
use std::thread;
pub struct FractalPicture {
    resolution: (u32, u32),
    seed: (f64, f64),
    pic: image::RgbImage,
    zoom_point: (u32, u32),
    path_name: String,
    zoom_level: f64,
    colorspace: ((u8, u8, u8), (u8, u8, u8)),
    thread_count: u32,
}

impl FractalPicture {
    pub fn new(
        resolution: (u32, u32),
        seed: (f64, f64),
        path_name: String,
        zoom_point: (i32, i32),
        zoom_level: f64,
        colorspace: ((u8, u8, u8), (u8, u8, u8)),
    ) -> FractalPicture {
        let thread_count: u32 = (num_cpus::get() / 2) as u32;
        let zoom_point = (
            ((resolution.0 / 2) as i32 + zoom_point.0) as u32,
            ((resolution.1 / 2) as i32 + zoom_point.1) as u32,
        );
        FractalPicture {
            resolution,
            seed,
            pic: image::ImageBuffer::new(resolution.0, resolution.1),
            path_name,
            zoom_point,
            zoom_level,
            colorspace,
            thread_count,
        }
    }

    pub fn render(&mut self) {
        let mut handels = Vec::new();

        let to_move = (
            self.resolution.0 / self.thread_count,
            self.resolution.1 / self.thread_count,
        );

        for x in 0..self.thread_count {
            for y in 0..self.thread_count {
                let zoom_point = self.zoom_point;
                let resolution = self.resolution;
                let seed = self.seed;
                let zoom_level = self.zoom_level;
                let colorspace = self.colorspace;
                let thread = thread::spawn(move || {
                    FractalPicture::renderer(
                        zoom_point,
                        resolution,
                        to_move,
                        (x * to_move.0, y * to_move.1),
                        zoom_level,
                        seed,
                        colorspace,
                    )
                });
                handels.push(thread);
            }
            println!("{}%", (x as f64 / self.thread_count as f64) * 100.0);
            for handle in handels {
                let erg = handle.join().unwrap();

                for pixel in erg {
                    self.pic.put_pixel(pixel.x, pixel.y, pixel.rgb);
                }
            }
            handels = Vec::new();
        }
    }

    pub fn save(&self) {
        self.pic
            .save_with_format(&self.path_name, ImageFormat::Png)
            .unwrap()
    }
}

impl FractalPicture {
    fn renderer(
        zoom_point: (u32, u32),
        resulution: (u32, u32),
        range: (u32, u32),
        starting_point: (u32, u32),
        zoom_level: f64,
        seed: (f64, f64),
        colorspace: ((u8, u8, u8), (u8, u8, u8)),
    ) -> Vec<RawImage> {
        let mut pixels: Vec<RawImage> = Vec::new();

        for x in starting_point.0..starting_point.0 + range.0 {
            for y in starting_point.1..starting_point.1 + range.1 {
                let erg = FractalPicture::julia_set(
                    ((x) as f64 - (resulution.0 + zoom_point.0 - resulution.0 / 2) as f64 / 2.0)
                        / (resulution.0 as f64 / 2.0)
                        * zoom_level,
                    ((y) as f64 - (resulution.1 + (zoom_point.1 - resulution.1 / 2)) as f64 / 2.0)
                        / (resulution.1 as f64 / 2.0)
                        * zoom_level,
                    seed,
                );

                pixels.push(RawImage::new(
                    x,
                    y,
                    FractalPicture::cool_pixel_stuff_pointythingi(
                        zoom_point,
                        resulution,
                        colorspace,
                        (x, y),
                        erg,
                        zoom_level,
                    ),
                ));
            }
        }
        pixels
    }

    fn julia_set(x: f64, y: f64, c: (f64, f64)) -> u8 {
        let mut z = Complex::new(x, y);
        let c = Complex::new(c.0, c.1);
        let mut i = 0;
        while i < 255 && z.norm() < 2.0 {
            z = z * z + c;
            i += 1;
        }
        i as u8
    }

    fn cool_pixel_stuff_pointythingi(
        zoom_point: (u32, u32),
        resulution: (u32, u32),
        colorspace: ((u8, u8, u8), (u8, u8, u8)),
        point: (u32, u32),
        in_erg: u8,
        zoom_level: f64,
    ) -> image::Rgb<u8> {
        if in_erg == 255 || in_erg == 0 {
            return image::Rgb([0, 0, 0]);
        } else {
            let dist_max =
                ((resulution.0 as f64 / 2.0).powi(2) + (resulution.1 as f64 / 2.0).powi(2)).sqrt();

            let disttozoompointfrommiddel = ((zoom_point.0 as f64 - resulution.0 as f64 / 2.0)
                .powi(2)
                + (zoom_point.1 as f64 - resulution.1 as f64 / 2.0).powi(2))
            .sqrt()
                * zoom_level;

            let distfromzoompointtopoint = ((zoom_point.0 as f64 - point.0 as f64).powi(2)
                + (zoom_point.1 as f64 - point.1 as f64).powi(2))
            .sqrt()
                * zoom_level;

            let disttotla = disttozoompointfrommiddel + distfromzoompointtopoint;

            let dist_rel = disttotla / dist_max;

            let color_original = LinSrgb::new(
                colorspace.0 .0 as f32 / 255.0,
                colorspace.0 .1 as f32 / 255.0,
                colorspace.0 .2 as f32 / 255.0,
            );

            let color_to_morph_to = LinSrgb::new(
                colorspace.1 .0 as f32 / 255.0,
                colorspace.1 .1 as f32 / 255.0,
                colorspace.1 .2 as f32 / 255.0,
            );

            //let g = (dist_rel * in_erg as f64) as u8;

            //let blue: u8 = ((255.0 * dist_rel) - (255 - g) as f64) as u8;

            let gradient =
                gradient::Gradient::new([color_original, color_to_morph_to]).get(dist_rel as f32);

            let ggg: [u8; 3] = Srgb::from_linear(gradient).into_format().into_raw();

            let ggg: [u8; 3] = [
                (ggg[0] as f64 * dist_rel) as u8,
                (ggg[2] as f64 + dist_rel) as u8,
                (ggg[1] as f64 + dist_rel) as u8,
            ];

            return image::Rgb(ggg);
        }
    }
}

struct RawImage {
    x: u32,
    y: u32,
    rgb: image::Rgb<u8>,
}

impl RawImage {
    pub fn new(x: u32, y: u32, rgb: image::Rgb<u8>) -> RawImage {
        RawImage { x, y, rgb }
    }
}
