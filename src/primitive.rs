use cgmath::Vector3;

pub type Color = [i32; 4];

#[derive(Debug)]
pub struct Sphere {
  pub center: Vector3<f32>,
  pub radius: f32,
  pub color: Color,
}

pub enum LightType {
  Ambient,
  Point,
  Directional,
}

pub struct Light {
  pub light_type: LightType,
  pub intensity: f32,
  pub position: Option<Vector3<f32>>, // ambientの場合はNoneが適切
}
