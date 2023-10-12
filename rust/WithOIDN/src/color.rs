use std::io::Write;

use super::vec3::Vec3;
use super::interval::Interval;

pub type Color = Vec3;

const INTENSITY: Interval = Interval{ min: 0.0, max: 1.0 };

pub fn linear_to_gamma(linear_component: f64) -> f64 {
  if linear_component > 0.0 {
    linear_component.sqrt()
  } else {
    0.0
  }
}

impl Color {
  pub fn write_color(&self, out: &mut dyn Write, samples_per_pixel: usize, is_linear: bool) -> std::io::Result<()> {
    let r = self.x();
    let g = self.y();
    let b = self.z();

    // Replace NaN components with zero.
    let r = if r.is_nan() { 0.0 } else { r };
    let g = if g.is_nan() { 0.0 } else { g };
    let b = if b.is_nan() { 0.0 } else { b };

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    let r = scale * r;
    let g = scale * g;
    let b = scale * b;

    // Apply the linear to gamma transform.
    let r = if is_linear { r } else { linear_to_gamma(r) };
    let g = if is_linear { g } else { linear_to_gamma(g) };
    let b = if is_linear { b } else { linear_to_gamma(b) };

    let r = INTENSITY.clamp(r) as f32;
    let g = INTENSITY.clamp(g) as f32;
    let b = INTENSITY.clamp(b) as f32;

    // Write the translated [0.0, 1.0] value of each color component.
    out.write_all(&r.to_le_bytes())?;
    out.write_all(&g.to_le_bytes())?;
    out.write_all(&b.to_le_bytes())?;

    Ok(())
  }
}
