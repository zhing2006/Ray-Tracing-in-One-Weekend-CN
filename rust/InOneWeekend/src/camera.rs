use super::rtweekend;
use super::color::Color;
use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::interval::Interval;
use super::vec3::{self, Point3, Vec3};

pub struct Camera {
  pub aspect_ratio: f64,  // Ratio of image width over height
  pub image_width: i32,   // Rendered image width in pixel count
  pub samples_per_pixel: usize, // Count of random samples for each pixel
  pub max_depth: i32,     // Maximum number of ray bounces into scene
  pub vfov: f64,          // Vertical field of view in degrees
  pub lookfrom: Point3,   // Camera origin
  pub lookat: Point3,     // Point camera is looking at
  pub vup: Vec3,          // Camera up vector
  pub defocus_angle: f64, // Defocus blur angle
  pub focus_dist: f64,    // Focus distance
  image_height: i32,      // Rendered image height
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
      vfov: 90.0,
      lookfrom: Point3::new(0.0, 0.0, -1.0),
      lookat: Point3::new(0.0, 0.0, 0.0),
      vup: Vec3::new(0.0, 1.0, 0.0),
      defocus_angle: 0.0,
      focus_dist: 10.0,
      image_height: 0,
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
  pub fn render(&mut self, world: &dyn Hittable) {
    self.initialize();

    println!("P3\n{} {}\n255", self.image_width, self.image_height);
    let stdout = std::io::stdout();

    for j in 0..self.image_height {
      eprintln!("\rScanlines remaining: {}", self.image_height - j);
      for i in 0..self.image_width {
        let mut pixel_color = Color::default();
        for _ in 0..self.samples_per_pixel {
          let r = self.get_ray(i, j);
          pixel_color += Self::ray_color(&r, self.max_depth, world);
        }
        pixel_color.write_color(&mut stdout.lock(), self.samples_per_pixel).unwrap();
      }
    }

    eprintln!("\nDone.");
  }

  fn initialize(&mut self) {
    self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
    self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

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

  fn get_ray(&self, i: i32, j: i32) -> Ray {
    // Get a randomly sampled camera ray for the pixel at location i,j.
    let pixel_center = self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
    let pixel_sample = pixel_center + self.pixel_sample_square();

    let ray_origin = if self.defocus_angle <= 0.0 {
      self.center
    } else {
      self.defocus_disk_sample()
    };
    let ray_direction = pixel_sample - ray_origin;

    Ray::new(ray_origin, ray_direction)
  }

  fn pixel_sample_square(&self) -> Vec3 {
    // Returns a random point in the square surrounding a pixel at the origin.
    let px = -0.5 + rtweekend::random_double();
    let py = -0.5 + rtweekend::random_double();
    px * self.pixel_delta_u + py * self.pixel_delta_v
  }

  fn defocus_disk_sample(&self) -> Point3 {
    // Returns a random point in the defocus disk.
    let p = vec3::random_in_unit_disk();
    self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
  }

  fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();

    // 如果我们超过了光线反弹限制，就不再收集光线。
    if depth <= 0 {
      return Color::default();
    }

    if world.hit(r, &Interval::new(0.001, rtweekend::INFINITY), &mut rec) {
      let mut scattered = Ray::default();
      let mut attenuation = Color::default();
      if let Some(mat) = rec.mat.clone() {
        if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
          return attenuation * Self::ray_color(&scattered, depth - 1, world);
        }
      }
      return Color::default();
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
  }
}