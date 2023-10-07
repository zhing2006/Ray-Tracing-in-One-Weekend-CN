use std::io::Write;

use super::vec3::Vec3;
use super::interval::Interval;

pub type Color = Vec3;

const INTENSITY: Interval = Interval{ min: 0.0, max: 0.999 };

pub fn linear_to_gamma(linear_component: f64) -> f64 {
  if linear_component > 0.0 {
    linear_component.sqrt()
  } else {
    0.0
  }
}

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

    // Apply the linear to gamma transform.
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    // Write the translated [0,255] value of each color component.
    writeln!(out, "{} {} {}",
      (256.0 * INTENSITY.clamp(r)) as i32,
      (256.0 * INTENSITY.clamp(g)) as i32,
      (256.0 * INTENSITY.clamp(b)) as i32)
  }
}
