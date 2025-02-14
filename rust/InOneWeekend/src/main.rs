pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod rtweekend;
pub mod interval;
pub mod camera;
pub mod material;

use std::rc::Rc;

use vec3::Point3;
use color::Color;
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;
use material::{
  Material,
  Lambertian,
  Metal,
  Dielectric,
};

fn main() {
  // World
  let mut world = HittableList::default();

  let ground_material: Rc<dyn Material> = Rc::new(
    Lambertian::new(color::Color::new(0.5, 0.5, 0.5))
  );
  world.add(Rc::new(
    Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)
  ));

  for a in -11..11 {
    for b in -11..11 {
      let choose_mat = rtweekend::random_double();
      let center = Point3::new(
        a as f64 + 0.9 * rtweekend::random_double(),
        0.2,
        b as f64 + 0.9 * rtweekend::random_double(),
      );

      if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
        let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
          // diffuse
          let albedo = Color::random() * Color::random();
          Rc::new(Lambertian::new(albedo))
        } else if choose_mat < 0.95 {
          // metal
          let albedo = Color::random_range(0.5, 1.0);
          let fuzz = rtweekend::random_double_range(0.0, 0.5);
          Rc::new(Metal::new(albedo, fuzz))
        } else {
          // glass
          Rc::new(Dielectric::new(1.5))
        };

        world.add(Rc::new(
          Sphere::new(center, 0.2, sphere_material)
        ));
      }
    }
  }

  let material1: Rc<dyn Material> = Rc::new(
    Dielectric::new(1.5)
  );
  world.add(Rc::new(
    Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)
  ));

  let material2: Rc<dyn Material> = Rc::new(
    Lambertian::new(Color::new(0.4, 0.2, 0.1))
  );
  world.add(Rc::new(
    Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)
  ));

  let material3: Rc<dyn Material> = Rc::new(
    Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)
  );
  world.add(Rc::new(
    Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)
  ));

  // Camera
  let mut cam = Camera::default();
  cam.aspect_ratio = 16.0 / 9.0;
  cam.image_width = 1200;
  cam.samples_per_pixel = 500;
  cam.max_depth = 50;

  cam.vfov = 20.0;
  cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
  cam.lookat = Point3::new(0.0, 0.0, 0.0);
  cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

  cam.defocus_angle = 0.6;
  cam.focus_dist = 10.0;

  // Render
  cam.render(&world);
}
