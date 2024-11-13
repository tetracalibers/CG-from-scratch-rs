use cgmath::Vector3;

pub type Color = [i32; 4];

pub fn scale_color(color: Color, intensity: f32) -> Color {
  // Scale color by intensity
  let mut color = color.map(|c| (c as f32 * intensity) as i32);
  // Set alpha to 255
  color.last_mut().map(|a| *a = 255);

  color
}

#[derive(Debug)]
pub struct Sphere {
  pub center: Vector3<f32>,
  pub radius: f32,
  pub color: Color,
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
