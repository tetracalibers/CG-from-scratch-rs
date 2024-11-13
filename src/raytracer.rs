use cgmath::{InnerSpace, Vector3};

use crate::primitive::{Color, Light, LightType, Sphere};

pub struct Scene<'a> {
  pub spheres: &'a [Sphere],
  pub background_color: Color,
  pub lights: &'a [Light],
}

impl<'a> Scene<'a> {
  pub fn new(spheres: &'a [Sphere], background_color: Color) -> Self {
    Self {
      spheres,
      background_color,
      lights: &[],
    }
  }

  pub fn with_lights(mut self, lights: &'a [Light]) -> Self {
    self.lights = lights;

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
      let mut L: Option<Vector3<f32>> = None;

      match light.ty {
        LightType::Ambient => {
          intensity += light.intensity;
        }
        LightType::Point(position) => {
          L = Some(position - P);
        }
        LightType::Directional(direction) => {
          L = Some(direction);
        }
      }

      if let Some(L) = L {
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
          let R = 2. * N * N.dot(L) - L;
          let r_dot_v = R.dot(V);

          if r_dot_v > 0.0 {
            intensity += light.intensity
              * (r_dot_v / (R.magnitude() * V.magnitude())).powf(specular);
          }
        }
      }
    }

    intensity
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
  ) -> Color {
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

    if let Some(sphere) = closest_sphere {
      if self.lights.is_empty() {
        return sphere.color;
      }

      let P = O + D * closest_t;
      let N = (P - sphere.center).normalize();

      let mut color =
        Vector3::new(sphere.color[0], sphere.color[1], sphere.color[2]);

      let intensity = self.compute_lighting(P, N, -D, sphere.specular);

      color = color * intensity;

      return [color.x, color.y, color.z, 255.];
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
