use cgmath::{InnerSpace, Vector3};

use crate::primitive::{Color, Light, LightType, Sphere};

pub struct Scene<'a> {
  pub spheres: &'a [Sphere],
  pub background_color: Color,
  pub lights: &'a [Light],
  pub shadow: bool,
}

impl<'a> Scene<'a> {
  pub fn new(spheres: &'a [Sphere], background_color: Color) -> Self {
    Self {
      spheres,
      background_color,
      lights: &[],
      shadow: false,
    }
  }

  pub fn with_lights(mut self, lights: &'a [Light]) -> Self {
    self.lights = lights;
    self
  }

  pub fn with_shadow(mut self) -> Self {
    self.shadow = true;
    self
  }

  /// * `P` - point
  /// * `N` - normal
  /// * `V` - view
  #[allow(non_snake_case, unused_assignments)]
  fn compute_lighting(
    &self,
    P: Vector3<f32>,
    N: Vector3<f32>,
    V: Vector3<f32>,
    specular: Option<f32>,
  ) -> f32 {
    let mut intensity = 0.0;

    for light in self.lights {
      let (L, t_max) = match light.ty {
        LightType::Ambient => {
          intensity += light.intensity;
          continue;
        }
        LightType::Point(position) => (position - P, 1.),
        LightType::Directional(direction) => (direction, f32::INFINITY),
      };

      //
      // Shadow check
      //

      if self.shadow {
        let blocker = self.closest_intersection(P, L, 0.001, t_max);

        if blocker.is_some() {
          continue;
        }
      }

      //
      // Diffuse reflection
      //

      let n_dot_l = N.dot(L);

      if n_dot_l > 0.0 {
        intensity +=
          light.intensity * n_dot_l / (N.magnitude() * L.magnitude());
      }

      //
      // Specular reflection
      //

      if let Some(specular) = specular {
        let R = self.reflect_ray(L, N);
        let r_dot_v = R.dot(V);

        if r_dot_v > 0.0 {
          intensity += light.intensity
            * (r_dot_v / (R.magnitude() * V.magnitude())).powf(specular);
        }
      }
    }

    intensity
  }

  /// * `R` - ray
  /// * `N` - normal
  #[allow(non_snake_case)]
  fn reflect_ray(&self, R: Vector3<f32>, N: Vector3<f32>) -> Vector3<f32> {
    2. * N * N.dot(R) - R
  }

  /// * `O` - origin
  /// * `D` - direction
  #[allow(non_snake_case)]
  fn closest_intersection(
    &self,
    O: Vector3<f32>,
    D: Vector3<f32>,
    min_t: f32,
    max_t: f32,
  ) -> Option<(&Sphere, f32)> {
    let mut closest_t = f32::INFINITY;
    let mut closest_sphere: Option<&Sphere> = None;

    for sphere in self.spheres {
      let (t1, t2) = intersect_ray_sphere(O, D, sphere);

      if t1 < closest_t && min_t < t1 && t1 < max_t {
        closest_t = t1;
        closest_sphere = Some(sphere);
      }

      if t2 < closest_t && min_t < t2 && t2 < max_t {
        closest_t = t2;
        closest_sphere = Some(sphere);
      }
    }

    closest_sphere.map(|sphere| (sphere, closest_t))
  }

  /// * `O` - origin
  /// * `D` - direction
  #[allow(non_snake_case)]
  pub fn trace_ray(
    &self,
    O: Vector3<f32>,
    D: Vector3<f32>,
    min_t: f32,
    max_t: f32,
    recursion_depth: Option<i32>,
  ) -> Color {
    let intersection = self.closest_intersection(O, D, min_t, max_t);
    if let Some((sphere, closest_t)) = intersection {
      if self.lights.is_empty() {
        return sphere.color;
      }

      let P = O + D * closest_t;
      let N = (P - sphere.center).normalize();

      let mut local_color =
        Vector3::new(sphere.color[0], sphere.color[1], sphere.color[2]);

      let intensity = self.compute_lighting(P, N, -D, sphere.specular);
      local_color = local_color * intensity;

      let recursion_depth = recursion_depth.unwrap_or(0);
      let r = sphere.reflective.unwrap_or(0.);

      if recursion_depth > 0 && r > 0. {
        let R = self.reflect_ray(-D, N);

        let reflected_color =
          self.trace_ray(P, R, 0.1, f32::INFINITY, Some(recursion_depth - 1));

        let reflected_color = Vector3::new(
          reflected_color[0],
          reflected_color[1],
          reflected_color[2],
        );

        local_color = local_color * (1. - r) + reflected_color * r;
      }

      return [local_color.x, local_color.y, local_color.z, 255.];
    }

    self.background_color
  }
}

/// * `O` - origin
/// * `D` - direction
#[allow(non_snake_case)]
fn intersect_ray_sphere(
  O: Vector3<f32>,
  D: Vector3<f32>,
  sphere: &Sphere,
) -> (f32, f32) {
  let r = sphere.radius;
  let CO = O - sphere.center;

  let k1 = D.dot(D);
  let k2 = 2. * CO.dot(D);
  let k3 = CO.dot(CO) - r * r;

  let discriminant = k2 * k2 - 4. * k1 * k3;

  if discriminant < 0. {
    return (f32::INFINITY, f32::INFINITY);
  }

  let t1 = (-k2 + discriminant.sqrt()) / (2. * k1);
  let t2 = (-k2 - discriminant.sqrt()) / (2. * k1);

  (t1, t2)
}
