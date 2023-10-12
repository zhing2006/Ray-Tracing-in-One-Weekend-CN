use std::io::Write;

use super::rtweekend;
use super::color::Color;
use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::interval::Interval;
use super::vec3::{self, Point3, Vec3};
use super::pdf::{
  Pdf,
  HittablePdf,
  MixturePdf,
};
use super::material::ScatterRecord;

pub struct Camera {
  pub aspect_ratio: f64,  // Ratio of image width over height
  pub image_width: usize, // Rendered image width in pixel count
  pub samples_per_pixel: usize, // Count of random samples for each pixel
  pub max_depth: usize,   // Maximum number of ray bounces into scene
  pub background: Color,  // Background color for rays that miss
  pub vfov: f64,          // Vertical field of view in degrees
  pub lookfrom: Point3,   // Camera origin
  pub lookat: Point3,     // Point camera is looking at
  pub vup: Vec3,          // Camera up vector
  pub defocus_angle: f64, // Defocus blur angle
  pub focus_dist: f64,    // Focus distance
  image_height: usize,    // Rendered image height
  sqrt_spp: usize,        // Square root of samples per pixel
  recip_sqrt_spp: f64,    // Reciprocal of square root of samples per pixel
  center: Point3,         // Camera center
  pixel00_loc: Point3,    // Location of pixel 0, 0
  pixel_delta_u: Vec3,    // Offset to pixel to the right
  pixel_delta_v: Vec3,    // Offset to pixel below
  u: Vec3,                // Camera horizontal axis
  v: Vec3,                // Camera vertical axis
  w: Vec3,                // Camera forward axis
  defocus_disk_u: Vec3,   // Defocus disk horizontal axis
  defocus_disk_v: Vec3,   // Defocus disk vertical axis
}

impl Default for Camera {
  fn default() -> Self {
    Self {
      aspect_ratio: 1.0,
      image_width: 100,
      samples_per_pixel: 10,
      max_depth: 10,
      background: Color::default(),
      vfov: 90.0,
      lookfrom: Point3::new(0.0, 0.0, -1.0),
      lookat: Point3::new(0.0, 0.0, 0.0),
      vup: Vec3::new(0.0, 1.0, 0.0),
      defocus_angle: 0.0,
      focus_dist: 10.0,
      image_height: 0,
      sqrt_spp: 10.0_f64.sqrt() as usize,
      recip_sqrt_spp: 1.0 / (10.0_f64.sqrt()),
      center: Point3::default(),
      pixel00_loc: Point3::default(),
      pixel_delta_u: Vec3::default(),
      pixel_delta_v: Vec3::default(),
      u: Vec3::default(),
      v: Vec3::default(),
      w: Vec3::default(),
      defocus_disk_u: Vec3::default(),
      defocus_disk_v: Vec3::default(),
    }
  }
}

impl Camera {
  pub fn render(&mut self, world: &dyn Hittable, lights: &dyn Hittable, output_file: &str, albedo_file: &str, normal_file: &str) {
    self.initialize();

    let mut output_file = std::fs::File::create(output_file).expect("Failed to create output file.");
    let mut albedo_file = std::fs::File::create(albedo_file).expect("Failed to create albedo file.");
    let mut normal_file = std::fs::File::create(normal_file).expect("Failed to create normal file.");
    writeln!(&mut output_file, "PF\n{} {}\n-1.0", self.image_width, self.image_height).expect("Failed to write header to output file.");
    writeln!(&mut albedo_file, "PF\n{} {}\n-1.0", self.image_width, self.image_height).expect("Failed to write header to albedo file.");
    writeln!(&mut normal_file, "PF\n{} {}\n-1.0", self.image_width, self.image_height).expect("Failed to write header to normal file.");

    for j in (0..self.image_height).rev() {
      eprintln!("\rScanlines processing: {}", self.image_height - j);
      for i in 0..self.image_width {
        let mut pixel_color = Color::default();
        let mut albedo_color = Color::default();
        let mut normal_color = Color::default();
        for s_j in 0..self.sqrt_spp {
          for s_i in 0..self.sqrt_spp {
            let r = self.get_ray(i as i32, j as i32, s_i as i32, s_j as i32);
            pixel_color += self.ray_color(&r, self.max_depth, world, lights);
            albedo_color += self.ray_albedo(&r, world);
            normal_color += self.ray_normal(&r, world);
          }
        }
        pixel_color.write_color(&mut output_file, self.samples_per_pixel, true).unwrap();
        albedo_color.write_color(&mut albedo_file, self.samples_per_pixel, true).unwrap();
        normal_color = 0.5 * (normal_color + Color::new(1.0, 1.0, 1.0));
        normal_color.write_color(&mut normal_file, self.samples_per_pixel, true).unwrap();
      }
    }

    eprintln!("\nDone.");
  }

  fn initialize(&mut self) {
    self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
    self.image_height = if self.image_height < 1 { 1 } else { self.image_height };
    self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as usize;
    self.recip_sqrt_spp = 1.0 / (self.sqrt_spp as f64);

    self.center = self.lookfrom;

    // 确定视口尺寸。
    let theta = rtweekend::degrees_to_radians(self.vfov);
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h * self.focus_dist;
    let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

    // 计算相机坐标系的 u,v,w 单位基向量。
    self.w = vec3::unit_vector(self.lookfrom - self.lookat);
    self.u = vec3::unit_vector(vec3::cross(self.vup, self.w));
    self.v = vec3::cross(self.w, self.u);

    // 计算水平和垂直视口边缘上的向量。
    let viewport_u = self.u * viewport_width;
    let viewport_v = -self.v * viewport_height;

    // 计算从像素到像素的水平和垂直增量向量。
    self.pixel_delta_u = viewport_u / self.image_width as f64;
    self.pixel_delta_v = viewport_v / self.image_height as f64;

    // 计算左上角像素的位置。
    let viewport_upper_left = self.center
      - (self.focus_dist * self.w)
      - (0.5 * viewport_u)
      - (0.5 * viewport_v);
    self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

    // 计算相机失焦盘的基向量。
    let defocus_radius = self.focus_dist * (rtweekend::degrees_to_radians(self.defocus_angle / 2.0)).tan();
    self.defocus_disk_u = self.u * defocus_radius;
    self.defocus_disk_v = self.v * defocus_radius;
  }

  // fn get_simple_ray(&self, i: i32, j: i32) -> Ray {
  //   let pixel_center = self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
  //   let ray_direction = pixel_center - self.center;
  //   Ray::new(self.center, ray_direction)
  // }

  fn get_ray(&self, i: i32, j: i32, s_i: i32, s_j: i32) -> Ray {
    // Get a randomly sampled camera ray for the pixel at location i,j.
    let pixel_center = self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
    let pixel_sample = pixel_center + self.pixel_sample_square(s_i, s_j);

    let ray_origin = if self.defocus_angle <= 0.0 {
      self.center
    } else {
      self.defocus_disk_sample()
    };
    let ray_direction = pixel_sample - ray_origin;
    let ray_time = rtweekend::random_double();

    Ray::new_with_time(ray_origin, ray_direction, ray_time)
  }

  fn pixel_sample_square(&self, s_i: i32, s_j: i32) -> Vec3 {
    // Returns a random point in the square surrounding a pixel at the origin.
    let px = -0.5 + self.recip_sqrt_spp * (s_i as f64 + rtweekend::random_double());
    let py = -0.5 + self.recip_sqrt_spp * (s_j as f64 + rtweekend::random_double());
    px * self.pixel_delta_u + py * self.pixel_delta_v
  }

  fn defocus_disk_sample(&self) -> Point3 {
    // Returns a random point in the defocus disk.
    let p = vec3::random_in_unit_disk();
    self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
  }

  fn ray_color(&self, r: &Ray, depth: usize, world: &dyn Hittable, lights: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();

    // 如果我们超过了光线反弹限制，就不再收集光线。
    if depth == 0 {
      return Color::default();
    }

    // 如果光线没有击中了世界中的任何东西，则返回背景颜色。
    if !world.hit(r, &Interval::new(0.001, rtweekend::INFINITY), &mut rec) {
      return self.background;
    }

    if let Some(mat) = rec.mat.clone() {
      let mut srec = ScatterRecord::default();
      let color_from_emission = mat.emitted(r, &rec, rec.u, rec.v, rec.p);

      if !mat.scatter(r, &rec, &mut srec) {
        return color_from_emission;
      }

      if srec.skip_pdf {
        return srec.attenuation * self.ray_color(&srec.skip_pdf_ray, depth - 1, world, lights);
      }

      let light_pdf = HittablePdf::new(lights, rec.p);
      let mixed_pdf = MixturePdf::new(&light_pdf, &*srec.pdf);

      let scattered = Ray::new_with_time(rec.p, mixed_pdf.generate(), r.time());
      let pdf = mixed_pdf.value(scattered.direction());

      let scattering_pdf = mat.scattering_pdf(r, &rec, &scattered);

      let sample_color = self.ray_color(&scattered, depth - 1, world, lights);
      let color_from_scatter = (srec.attenuation * scattering_pdf * sample_color) / pdf;

      color_from_emission + color_from_scatter
    } else {
      Color::default()
    }
  }

  fn ray_albedo(&self, r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();

    // 如果光线没有击中了世界中的任何东西，则返回背景颜色。
    if !world.hit(r, &Interval::new(0.001, rtweekend::INFINITY), &mut rec) {
      return self.background;
    }

    if let Some(mat) = rec.mat.clone() {
      mat.albedo(r, &rec, rec.u, rec.v, rec.p)
    } else {
      Color::default()
    }
  }

  fn ray_normal(&self, r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();

    // 如果光线没有击中了世界中的任何东西，则返回背景颜色。
    if !world.hit(r, &Interval::new(0.001, rtweekend::INFINITY), &mut rec) {
      return self.background;
    }

    if let Some(mat) = rec.mat.clone() {
      mat.normal(r, &rec, rec.u, rec.v, rec.p)
    } else {
      Color::default()
    }
  }
}