use std::sync::Arc;

use anyhow::Result;

use winit::{
  application::ApplicationHandler,
  dpi::{LogicalSize, PhysicalSize},
  event::{ElementState, KeyEvent, WindowEvent},
  event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
  keyboard::{Key, NamedKey},
  window::{Window, WindowId},
};

pub trait Render: Sized {
  fn new(window: &Window) -> Result<Self>;
  fn resize(&mut self, size: PhysicalSize<u32>) -> Result<()>;
  fn draw(&mut self) -> Result<()>;
}

pub struct App<'a, R: Render> {
  window: Option<Arc<Window>>,
  window_title: &'a str,
  window_size: Option<LogicalSize<u32>>,

  request_redraw: bool,
  close_requested: bool,

  renderer: Option<R>,
}

impl<'a, R: Render> App<'a, R> {
  pub fn new(window_title: &'a str) -> Self {
    Self {
      window: None,
      window_title,
      window_size: None,

      request_redraw: true,
      close_requested: false,

      renderer: None,
    }
  }

  pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
    self.window_size = Some(LogicalSize::new(width, height));
    self
  }

  pub fn run(&mut self) -> Result<()> {
    let event_loop = EventLoop::builder().build()?;
    event_loop.run_app(self)?;

    Ok(())
  }

  fn window(&self) -> Option<&Window> {
    match &self.window {
      Some(window) => Some(window.as_ref()),
      None => None,
    }
  }

  fn init(&mut self, window: Arc<Window>) -> Result<()> {
    let renderer = R::new(window.as_ref())?;
    self.renderer = Some(renderer);
    Ok(())
  }
}

impl<'a, R: Render> ApplicationHandler for App<'a, R> {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let mut window_attributes =
      Window::default_attributes().with_title(self.window_title);

    if let Some(window_size) = self.window_size {
      window_attributes = window_attributes.with_max_inner_size(window_size);
    }

    let window = event_loop.create_window(window_attributes).unwrap();
    self.window = Some(Arc::new(window));

    match self.init(self.window.as_ref().unwrap().clone()) {
      Ok(_) => (),
      Err(e) => eprintln!("{:?}", e),
    }
  }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    window_id: WindowId,
    event: WindowEvent,
  ) {
    let binding = self.window();
    let window = match &binding {
      Some(window) => window,
      None => return,
    };
    if window.id() != window_id {
      return;
    }

    let renderer = match &mut self.renderer {
      Some(renderer) => renderer,
      None => return,
    };

    match event {
      WindowEvent::Resized(size) => match renderer.resize(size) {
        Ok(_) => (),
        Err(e) => eprintln!("{:?}", e),
      },
      WindowEvent::RedrawRequested => {
        let result = renderer.draw();

        match result {
          Ok(_) => event_loop.set_control_flow(ControlFlow::Wait),
          Err(e) => eprintln!("{:?}", e),
        }
      }
      WindowEvent::CloseRequested => {
        self.close_requested = true;
      }
      WindowEvent::KeyboardInput {
        event:
          KeyEvent {
            logical_key: key,
            state: ElementState::Pressed,
            ..
          },
        ..
      } => match key.as_ref() {
        Key::Character("r") => {
          self.request_redraw = !self.request_redraw;
        }
        Key::Named(NamedKey::Escape) => {
          self.close_requested = true;
        }
        _ => (),
      },
      _ => {}
    }
  }

  fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
    if self.request_redraw && !self.close_requested {
      let binding = self.window();
      let window = match &binding {
        Some(window) => window,
        None => return,
      };
      window.request_redraw();
    }

    if self.close_requested {
      event_loop.exit();
    }
  }
}
