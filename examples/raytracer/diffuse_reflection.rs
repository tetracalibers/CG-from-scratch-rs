use anyhow::Result;
use cgmath::Vector3;
use computer_graphics_from_scratch_rs::canvas::Canvas;
use computer_graphics_from_scratch_rs::export::export_png;
use computer_graphics_from_scratch_rs::primitive::Color;
use computer_graphics_from_scratch_rs::primitive::Direction;
use computer_graphics_from_scratch_rs::primitive::Position;
use computer_graphics_from_scratch_rs::primitive::Sphere;
use computer_graphics_from_scratch_rs::primitive::{Light, LightType};
use computer_graphics_from_scratch_rs::raytracer::Scene;

const EXPORT_PATH: &str = "export/raytracer_02_diffuse_reflection.png";

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
  Sphere {
    center: Vector3::new(0., -5001., 0.),
    radius: 5000.,
    color: [255., 255., 0., 255.],
    specular: None,
    reflective: None,
  },
];

const LIGHTS: &[Light] = &[
  Light {
    ty: LightType::Ambient,
    intensity: 0.2,
  },
  Light {
    ty: LightType::Point(Position::new(2., 1., 0.)),
    intensity: 0.6,
  },
  Light {
    ty: LightType::Directional(Direction::new(1., 4., 4.)),
    intensity: 0.2,
  },
];

fn main() -> Result<()> {
  let mut canvas = Canvas::new(
    CANVAS_WIDTH,
    CANVAS_HEIGHT,
    VIEWPORT_SIZE,
    PROJECTION_PLANE_Z,
  );
  let scene = Scene::new(SPHERES, BACKGROUND_COLOR).with_lights(LIGHTS);

  let cw = CANVAS_WIDTH as i32;
  let ch = CANVAS_HEIGHT as i32;

  for x in -cw / 2..cw / 2 {
    for y in -ch / 2..ch / 2 {
      let direction = canvas.canvas_to_viewport(x as f32, y as f32);
      let color =
        scene.trace_ray(CAMERA_POSITION, direction, 1., f32::INFINITY);

      canvas.put_pixel(x as f32, y as f32, color);
    }
  }

  export_png(EXPORT_PATH, canvas.data(), CANVAS_WIDTH, CANVAS_HEIGHT)?;

  Ok(())
}
