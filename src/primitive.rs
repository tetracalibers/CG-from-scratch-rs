use cgmath::Vector3;

pub type Color = [f32; 4];

#[derive(Debug)]
pub struct Sphere {
  pub center: Vector3<f32>,
  pub radius: f32,
  pub color: Color,
  pub specular: Option<f32>,
}

pub type Position = Vector3<f32>;
pub type Direction = Vector3<f32>;

pub enum LightType {
  Ambient,
  Point(Position),
  Directional(Direction),
}

pub struct Light {
  pub ty: LightType,
  pub intensity: f32,
}
