use std::rc::Rc;

use super::vec3::Point3;
use super::color::Color;
use super::rtw_stb_image::RtwImage;
use super::interval::Interval;

pub trait Texture {
  fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub struct SolidColor {
  color_value: Color,
}

impl SolidColor {
  pub fn new(color_value: Color) -> Self {
    Self {
      color_value,
    }
  }

  pub fn new_with_rgb(red: f64, green: f64, blue: f64) -> Self {
    Self {
      color_value: Color::new(red, green, blue),
    }
  }
}

impl Texture for SolidColor {
  fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
    self.color_value
  }
}

pub struct CheckerTexture {
  inv_scale: f64,
  even: Rc<dyn Texture>,
  odd: Rc<dyn Texture>,
}

impl CheckerTexture {
  pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
    Self {
      inv_scale: 1.0 / scale,
      even,
      odd,
    }
  }

  pub fn new_with_color(scale: f64, c1: Color, c2: Color) -> Self {
    Self {
      inv_scale: 1.0 / scale,
      even: Rc::new(SolidColor::new(c1)),
      odd: Rc::new(SolidColor::new(c2)),
    }
  }
}

impl Texture for CheckerTexture {
  fn value(&self, u: f64, v: f64, p: Point3) -> Color {
    let x_integer = (self.inv_scale * p.x()).floor() as i32;
    let y_integer = (self.inv_scale * p.y()).floor() as i32;
    let z_integer = (self.inv_scale * p.z()).floor() as i32;

    let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

    if is_even {
      self.even.value(u, v, p)
    } else {
      self.odd.value(u, v, p)
    }
  }
}

pub struct ImageTexture {
  image: RtwImage,
}

impl ImageTexture {
  pub fn new(filename: &str) -> Self {
    Self {
      image: RtwImage::new(filename),
    }
  }
}

impl Texture for ImageTexture {
  fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
    // 如果没有纹理数据，则返回固定的青色作为调试辅助。
    if self.image.height() == 0 {
      return Color::new(0.0, 1.0, 1.0);
    }

    // 将输入的纹理坐标限制在 [0,1] x [1,0] 范围内
    let u = Interval::new(0.0, 1.0).clamp(u);
    let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

    let i = (u * self.image.width() as f64) as usize;
    let j = (v * self.image.height() as f64) as usize;
    let pixel = self.image.pixel_data(i, j);

    let color_scale = 1.0 / 255.0;
    Color::new(
      color_scale * pixel[0] as f64,
      color_scale * pixel[1] as f64,
      color_scale * pixel[2] as f64,
    )
  }
}