use cgmath::Vector3;

use crate::primitive::Color;

pub struct Canvas {
  viewport_size: f32,
  projection_plane_z: f32,

  canvas_width: f32,
  canvas_height: f32,

  data: Vec<u8>,
}

impl Canvas {
  pub fn new(
    canvas_width: u32,
    canvas_height: u32,
    viewport_size: f32,
    projection_plane_z: f32,
  ) -> Self {
    Self {
      viewport_size,
      projection_plane_z,
      canvas_width: canvas_width as f32,
      canvas_height: canvas_height as f32,
      data: vec![0; (canvas_width * canvas_height * 4) as usize],
    }
  }

  pub fn canvas_to_viewport(&self, x: f32, y: f32) -> Vector3<f32> {
    let x = x * self.viewport_size / self.canvas_width;
    let y = y * self.viewport_size / self.canvas_height;
    let z = self.projection_plane_z;

    Vector3::new(x, y, z)
  }

  pub fn put_pixel(&mut self, x: f32, y: f32, color: Color) {
    let w = self.canvas_width;
    let h = self.canvas_height;

    let x = w / 2. + x;
    let y = h / 2. - y - 1.;

    if x < 0. || y < 0. || x >= w || y >= h {
      return;
    }

    let offset = (4. * (x + y * w)) as usize;

    self.data[offset] = color[0] as u8;
    self.data[offset + 1] = color[1] as u8;
    self.data[offset + 2] = color[2] as u8;
    self.data[offset + 3] = color[3] as u8;
  }

  pub fn data(&self) -> &[u8] {
    &self.data
  }
}
