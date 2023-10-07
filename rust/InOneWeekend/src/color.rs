use std::io::Write;

use super::vec3::Vec3;

pub type Color = Vec3;

impl Color {
  pub fn write_color(&self, out: &mut dyn Write) -> std::io::Result<()> {
    write!(out, "{} {} {}",
      (255.999 * self.x()) as i32,
      (255.999 * self.y()) as i32,
      (255.999 * self.z()) as i32)
  }
}
