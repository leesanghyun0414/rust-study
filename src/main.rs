use mandelbrot::core::{parse_complex, parse_pair, pixel_to_point, render, write_image};

extern crate rayon;
use rayon::prelude::*;
/// test
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let bounds = parse_pair(&args[2], 'x').expect("t");
    let upper_left = parse_complex(&args[3]).expect("");
    let lower_right = parse_complex(&args[4]).expect("");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    let machine_type = &args[5];

    if machine_type == "1" {
        let threads = 8;
        let rows_per_band = bounds.1 / threads + 1;
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .expect("TODO: panic message");
        write_image(&args[1], &pixels, bounds).expect("TODO: panic message");
    } else if machine_type == "3" {
        render(&mut pixels, bounds, upper_left, lower_right);
        write_image(&args[1], &pixels, bounds).expect("TODO: panic message");
    } else {
        let bands: Vec<(usize, &mut [u8])> = pixels.chunks_mut(bounds.0).enumerate().collect();
        bands.into_par_iter().for_each(|(i, band)| {
            let top = i;
            let band_bounds = (bounds.0, 1);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right = pixel_to_point(bounds, (0, top + 1), upper_left, lower_right);
            render(band, band_bounds, band_upper_left, band_lower_right);
        });
        write_image(&args[1], &pixels, bounds).expect("TODO: panic message");
    }
}
