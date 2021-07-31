use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    let mut file = File::create("test.ppm")?;

    writeln!(file, "P3")?;
    writeln!(file, "# The P3 means colors are in ASCII")?;
    writeln!(file, "# 3 columns, 2 rows")?;
    writeln!(file, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(file, "# Max color")?;
    writeln!(file, "255")?;
    writeln!(file, "# Color triplets")?;

    let lol = Vec3::default();

    let bar = ProgressBar::new(IMAGE_HEIGHT as _);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );

    for j in (0..IMAGE_HEIGHT).rev() {
        bar.inc(1);
        for i in 0..IMAGE_WIDTH {
            //let color = Color::
            let r = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let g = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let ir = (255.299 * r) as u8;
            let ig = (255.299 * g) as u8;
            let ib = (255.299 * b) as u8;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    bar.finish();
    Ok(())
}
