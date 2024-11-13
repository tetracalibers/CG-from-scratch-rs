use anyhow::Result;
use computer_graphics_from_scratch_rs::winit_app::{App, Render};
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

const BOX_SIZE: i16 = 64;

fn main() -> Result<()> {
  env_logger::init();

  let mut app: App<State> =
    App::new("Hello pixels!").with_window_size(WIDTH, HEIGHT);
  app.run()?;

  Ok(())
}

struct World {
  pub box_x: i16,
  pub box_y: i16,
}
impl Default for World {
  fn default() -> Self {
    Self {
      box_x: 24,
      box_y: 16,
    }
  }
}

struct State {
  pixels: Pixels,
  world: World,
}

impl Render for State {
  fn new(window: &Window) -> Result<Self> {
    let pixels = {
      let window_size = window.inner_size();

      let surface_texture =
        SurfaceTexture::new(window_size.width, window_size.height, window);

      Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    Ok(Self {
      pixels,
      world: World::default(),
    })
  }

  fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) -> Result<()> {
    self.pixels.resize_surface(size.width, size.height)?;

    Ok(())
  }

  fn draw(&mut self) -> Result<()> {
    let frame = self.pixels.frame_mut();

    let World { box_x, box_y, .. } = self.world;

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
      let x = (i % WIDTH as usize) as i16;
      let y = (i / WIDTH as usize) as i16;

      let inside_the_box = x >= box_x
        && x < box_x + BOX_SIZE
        && y >= box_y
        && y < box_y + BOX_SIZE;

      let rgba = if inside_the_box {
        [0x5e, 0x48, 0xe8, 0xff]
      } else {
        [0x48, 0xb2, 0xe8, 0xff]
      };

      pixel.copy_from_slice(&rgba);
    }

    self.pixels.render()?;

    Ok(())
  }
}
