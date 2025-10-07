use std::io::Write;

use crate::common::clamp;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(
    out: &mut impl Write,
    pixel_color: Color,
    samples_per_pixel: i32
) {
    let scale = 1.0 / samples_per_pixel as f64;

    // sqrt for gamma correction
    let r = f64::sqrt(pixel_color.x() * scale);
    let g = f64::sqrt(pixel_color.y() * scale);
    let b = f64::sqrt(pixel_color.z() * scale);

    writeln!(
        out,
        "{} {} {}",
        (256.0 * clamp(0.0, r, 0.999)) as i32,
        (256.0 * clamp(0.0, g, 0.999)) as i32,
        (256.0 * clamp(0.0, b, 0.999)) as i32,
    )
    .expect("write color");
}
