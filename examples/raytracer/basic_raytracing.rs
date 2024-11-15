use anyhow::Result;
use cgmath::Vector3;
use computer_graphics_from_scratch_rs::{
  canvas::Canvas,
  export::export_png,
  primitive::{Color, Sphere},
  raytracer::Scene,
};

const EXPORT_PATH: &str = "export/raytracer_01_basic_raytracing.png";

const CANVAS_WIDTH: u32 = 1200;
const CANVAS_HEIGHT: u32 = 1200;

const VIEWPORT_SIZE: f32 = 1.;
const PROJECTION_PLANE_Z: f32 = 1.;

const CAMERA_POSITION: Vector3<f32> = Vector3::new(0., 0., 0.);

const BACKGROUND_COLOR: Color = [255., 255., 255., 255.];

const SPHERES: &[Sphere] = &[
  Sphere {
    center: Vector3::new(0., -1., 3.),
    radius: 1.,
    color: [255., 0., 0., 255.],
    specular: None,
    reflective: None,
  },
  Sphere {
    center: Vector3::new(-2., 0., 4.),
    radius: 1.,
    color: [0., 255., 0., 255.],
    specular: None,
    reflective: None,
  },
  Sphere {
    center: Vector3::new(2., 0., 4.),
    radius: 1.,
    color: [0., 0., 255., 255.],
    specular: None,
    reflective: None,
  },
];

fn main() -> Result<()> {
  let mut canvas = Canvas::new(
    CANVAS_WIDTH,
    CANVAS_HEIGHT,
    VIEWPORT_SIZE,
    PROJECTION_PLANE_Z,
  );
  let scene = Scene::new(SPHERES, BACKGROUND_COLOR);

  let cw = CANVAS_WIDTH as i32;
  let ch = CANVAS_HEIGHT as i32;

  for x in -cw / 2..cw / 2 {
    for y in -ch / 2..ch / 2 {
      let direction = canvas.canvas_to_viewport(x as f32, y as f32);
      let color =
        scene.trace_ray(CAMERA_POSITION, direction, 1., f32::INFINITY, None);

      canvas.put_pixel(x as f32, y as f32, color);
    }
  }

  export_png(EXPORT_PATH, canvas.data(), CANVAS_WIDTH, CANVAS_HEIGHT)?;

  Ok(())
}
