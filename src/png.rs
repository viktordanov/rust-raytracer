use std::{path::Path, fs::File, io::BufWriter};

use crate::pixel::{Image, Persistable, image_to_byte_array};

impl Persistable for Image {
    fn save(&self, path: &str, width: u32, height: u32) -> Result<(), std::io::Error> {
        let path = Path::new(path);
        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_trns(vec![0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8]);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(
            // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header()?;

		let bytes = image_to_byte_array(&self);
        match writer.write_image_data(&bytes) {
			Ok(_) => Ok(()),
			Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
		}
    }
}
