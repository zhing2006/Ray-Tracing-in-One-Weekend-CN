# 使用Rust实现RayTracingTheNextWeek

## 初始化

使用`cargo`创建工程

    cargo new --name the_next_week TheNextWeek



## 运动模糊

### SpaceTime光线追踪简介

```rust
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
+   tm: f64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self {
            orig,
            dir,
+           tm: 0.0,
        }
    }

+   pub fn new_with_time(orig: Point3, dir: Vec3, tm: f64) -> Self {
+       Self {
+           orig,
+           dir,
+           tm,
+       }
+   }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

+   pub fn time(&self) -> f64 {
+       self.tm
+   }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
```
Listing 1: [ray.h] 带有时间信息的光线


### 更新相机以模拟运动模糊

```rust
impl Camera {
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
+       let ray_time = rtweekend::random_double();

+       Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }
}
```
Listing 2: [camera.h] 带有时间信息的相机

```rust
pub struct Sphere {
+   center1: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
+   is_moving: bool,
+   center_vec: Vec3,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
+           center1: center,
            radius,
            mat: material,
+           is_moving: false,
+           center_vec: Vec3::default(),
        }
    }

+   pub fn new_with_center2(center1: Point3, center2: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
+       Self {
+           center1,
+           radius,
+           mat: material,
+           is_moving: true,
+           center_vec: center2 - center1,
+       }
+   }
+
+   fn sphere_center(&self, time: f64) -> Point3 {
+       self.center1 + self.center_vec * time
+   }

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        ...

        hit_record.t = root;
        hit_record.p = r.at(hit_record.t);
+       let outward_normal = (hit_record.p - self.center1) / self.radius;
        hit_record.set_face_normal(r, outward_normal);
        hit_record.mat = Some(Rc::clone(&self.mat));

        true
    }
}
```
Listing 3: [sphere.h] 移动的球体

```rust
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
+       let center = if self.is_moving { self.sphere_center(r.time()) } else { self.center1 };
+       let oc = center - r.origin();
        let a = r.direction().length_squared();
        let h = vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        ...
    }
}
```
Listing 4: [sphere.h] 移动的球体的碰撞函数


### 追踪光线交点的时间

```rust
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();

        // 捕捉退化的散射方向
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

+       *scattered = Ray::new_with_time(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
+       *scattered = Ray::new_with_time(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo;
        vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        ...
+       *scattered = Ray::new_with_time(rec.p, direction, r_in.time());
        true
    }
}
```
Listing 5: [material.h] 在 material::scatter() 方法中处理光线时间


### 将所有内容放在一起

```rust
fn main() {
    ...

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

+               let center2 = center + vec3::Vec3::new(0.0, rtweekend::random_double_range(0.0, 0.5), 0.0);
                world.add(Rc::new(
+                   Sphere::new_with_center2(center, center2, 0.2, sphere_material)
                ));
            }
        }
    }

    ...
}
```
Listing 6: [main.cc] 上一本书的最终场景，但是球会移动

![图像 1: 弹跳的球](../../images/img-2.01-bouncing-spheres.png)