use super::vec3::{
  Vec3,
  Point3,
};

#[derive(Default, Debug)]
pub struct Ray {
  orig: Vec3,
  dir: Vec3,
  tm: f64,
}

impl Ray {
  pub fn new(orig: Point3, dir: Vec3) -> Self {
    Self {
      orig,
      dir,
      tm: 0.0,
    }
  }

  pub fn new_with_time(orig: Point3, dir: Vec3, tm: f64) -> Self {
    Self {
      orig,
      dir,
      tm,
    }
  }

  pub fn origin(&self) -> Point3 {
    self.orig
  }

  pub fn direction(&self) -> Vec3 {
    self.dir
  }

  pub fn time(&self) -> f64 {
    self.tm
  }

  pub fn at(&self, t: f64) -> Point3 {
    self.orig + self.dir * t
  }
}