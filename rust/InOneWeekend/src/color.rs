use std::io::Write;

use super::vec3::Vec3;
use super::interval::Interval;

pub type Color = Vec3;

const INTENSITY: Interval = Interval{ min: 0.0, max: 0.999 };

impl Color {
  pub fn write_color(&self, out: &mut dyn Write, samples_per_pixel: usize) -> std::io::Result<()> {
    let r = self.x();
    let g = self.y();
    let b = self.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    let r = scale * r;
    let g = scale * g;
    let b = scale * b;

    // Write the translated [0,255] value of each color component.
    writeln!(out, "{} {} {}",
      (256.0 * INTENSITY.clamp(r)) as i32,
      (256.0 * INTENSITY.clamp(g)) as i32,
      (256.0 * INTENSITY.clamp(b)) as i32)
  }
}
