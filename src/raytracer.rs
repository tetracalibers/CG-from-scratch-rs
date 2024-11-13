use cgmath::{ElementWise, InnerSpace, Vector3};

use crate::primitive::{Color, Sphere};

pub struct Scene<'a> {
  pub spheres: &'a [Sphere],
  pub background_color: Color,
}

impl<'a> Scene<'a> {
  pub fn new(spheres: &'a [Sphere], background_color: Color) -> Self {
    Self {
      spheres,
      background_color,
    }
  }

  pub fn trace_ray(
    &self,
    origin: Vector3<f32>,
    direction: Vector3<f32>,
    min_t: f32,
    max_t: f32,
  ) -> Color {
    let mut closest_t = f32::INFINITY;
    let mut closest_sphere: Option<&Sphere> = None;

    for sphere in self.spheres {
      let (t1, t2) = intersect_ray_sphere(origin, direction, sphere);

      if t1 < closest_t && min_t < t1 && t1 < max_t {
        closest_t = t1;
        closest_sphere = Some(sphere);
      }

      if t2 < closest_t && min_t < t2 && t2 < max_t {
        closest_t = t2;
        closest_sphere = Some(sphere);
      }
    }

    if let Some(sphere) = closest_sphere {
      return sphere.color;
    }

    self.background_color
  }
}

fn intersect_ray_sphere(
  origin: Vector3<f32>,
  direction: Vector3<f32>,
  sphere: &Sphere,
) -> (f32, f32) {
  let oc = origin.sub_element_wise(sphere.center);

  let k1 = direction.dot(direction);
  let k2 = 2. * oc.dot(direction);
  let k3 = (oc.dot(oc) - sphere.radius * sphere.radius) as f32;

  let discriminant = k2 * k2 - 4. * k1 * k3;

  if discriminant < 0. {
    return (f32::INFINITY, f32::INFINITY);
  }

  let t1 = (-k2 + discriminant.sqrt()) / (2. * k1);
  let t2 = (-k2 - discriminant.sqrt()) / (2. * k1);

  (t1, t2)
}
