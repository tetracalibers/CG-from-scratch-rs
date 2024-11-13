use cgmath::Vector3;

pub type Color = [i32; 4];

#[derive(Debug)]
pub struct Sphere {
  pub center: Vector3<f32>,
  pub radius: f32,
  pub color: Color,
}
