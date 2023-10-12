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
pub mod aabb;
pub mod bvh;
pub mod texture;
pub mod rtw_stb_image;
pub mod perlin;
pub mod quad;
pub mod constant_medium;
pub mod onb;
pub mod pdf;

use std::rc::Rc;

use vec3::{Vec3, Point3};
use color::Color;
use hittable_list::HittableList;
use camera::Camera;
use material::{
  Material,
  Dielectric,
  Lambertian,
  DiffuseLight,
};
use quad::{
  Quad,
  make_box,
};
use hittable::{
  Translate,
  RotateY,
};
use sphere::Sphere;

fn cornell_box() {
  let mut world = HittableList::default();

  let red: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
  let white: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
  let green: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
  let light: Rc<dyn Material> = Rc::new(DiffuseLight::new_with_color(Color::new(15.0, 15.0, 15.0)));

  world.add(Rc::new(
    Quad::new(
      Point3::new(555.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 555.0, 0.0),
      vec3::Vec3::new(0.0, 0.0, 555.0),
      green
    )
  ));
  world.add(Rc::new(
    Quad::new(
      Point3::new(0.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 555.0, 0.0),
      vec3::Vec3::new(0.0, 0.0, 555.0),
      red
    )
  ));
  world.add(Rc::new(
    Quad::new(
      Point3::new(343.0, 554.0, 332.0),
      vec3::Vec3::new(-130.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 0.0, -105.0),
      Rc::clone(&light)
    )
  ));
  world.add(Rc::new(
    Quad::new(
      Point3::new(0.0, 0.0, 0.0),
      vec3::Vec3::new(555.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 0.0, 555.0),
      Rc::clone(&white)
    )
  ));
  world.add(Rc::new(
    Quad::new(
      Point3::new(555.0, 555.0, 555.0),
      vec3::Vec3::new(-555.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 0.0, -555.0),
      Rc::clone(&white)
    )
  ));
  world.add(Rc::new(
    Quad::new(
      Point3::new(0.0, 0.0, 555.0),
      vec3::Vec3::new(555.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 555.0, 0.0),
      Rc::clone(&white)
    )
  ));

  let box1 = make_box(
    Point3::new(0.0, 0.0, 0.0),
    Vec3::new(165.0, 330.0, 165.0),
    Rc::clone(&white),
  );
  let box1 = Rc::new(RotateY::new(box1, 15.0));
  let box1 = Rc::new(Translate::new(box1, vec3::Vec3::new(265.0, 0.0, 295.0)));
  world.add(box1);

  let glass: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
  world.add(Rc::new(
    Sphere::new(
      Point3::new(190.0, 90.0, 190.0),
      90.0,
      Rc::clone(&glass)
    )
  ));

  // Light Sources.
  let mut lights = HittableList::default();
  lights.add(Rc::new(
    Quad::new(
      Point3::new(343.0, 554.0, 332.0),
      vec3::Vec3::new(-130.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 0.0, -105.0),
      Rc::clone(&light),
    )
  ));
  lights.add(Rc::new(
    Sphere::new(
      Point3::new(190.0, 90.0, 190.0),
      90.0,
      Rc::clone(&glass),
    )
  ));

  let mut cam = Camera::default();

  cam.aspect_ratio = 1.0;
  cam.image_width = 400;
  cam.samples_per_pixel = 100;
  cam.max_depth = 10;
  cam.background = Color::default();

  cam.vfov = 40.0;
  cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
  cam.lookat = Point3::new(278.0, 278.0, 0.0);
  cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

  cam.defocus_angle = 0.0;

  cam.render(&world, &lights);
}

fn main() {
  let now = std::time::Instant::now();

  cornell_box();

  let elapsed = now.elapsed();
  eprintln!("Elapsed: {}.{:03}s", elapsed.as_secs(), elapsed.subsec_millis());
}