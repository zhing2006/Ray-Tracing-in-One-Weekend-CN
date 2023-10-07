use super::vec3::{
  Vec3,
  Point3,
};

#[derive(Default)]
pub struct Ray {
  orig: Vec3,
  dir: Vec3,
}

impl Ray {
  pub fn new(orig: &Point3, dir: &Vec3) -> Self {
    Self {
      orig: *orig,
      dir: *dir,
    }
  }

  pub fn origin(&self) -> &Point3 {
    &self.orig
  }

  pub fn direction(&self) -> &Vec3 {
    &self.dir
  }

  pub fn at(&self, t: f64) -> Point3 {
    self.orig + self.dir * t
  }
}