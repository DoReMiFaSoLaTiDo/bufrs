use std::sync::Arc;
use winit::{
  application::ApplicationHandler,
  event::*,
  event_loop::ActiveEventLoop,
  keyboard::{Key, NamedKey},
  window::{Window, WindowId}
};

use crate::my_lib::State;

#[derive(Default)]
pub struct App<'window> {
  window: Option<Arc<Window>>,
  object_state: Option<State<'window>>,
  // a toggle flag used to control the size of the surface
  flag: bool,
}

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

impl<'window> ApplicationHandler for App<'window> {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    if self.window.is_none() {
      let win_attr = Window::default_attributes().with_title("App Initialization");
      // use Arc.
      let window = Arc::new(
        event_loop.create_window(win_attr).expect("create window err."),
      );
      self.window = Some(window.clone());
      let object_state = State::new(window.clone());
      self.object_state = Some(object_state);
    }
  }

  // fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
  //   self.window_event(event_loop, window_id, event)
  // }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: WindowId,
    event: WindowEvent,
  ) {
    match event {
      WindowEvent::CloseRequested => {
        event_loop.exit();
      }
      WindowEvent::Resized(new_size) => {
        if let (Some(object_state), Some(window)) =  (self.object_state.as_mut(), self.window.as_ref()) {
          object_state.resize((new_size.width, new_size.height).into());
          window.request_redraw();
        }
      }
      WindowEvent::KeyboardInput {
        event: KeyEvent {
          repeat: false,
          state: ElementState::Pressed,
          logical_key: Key::Named(NamedKey::Enter),
          ..
        },
        ..
      } => {
        if let (Some(object_state), Some(window)) = (self.object_state.as_mut(), self.window.as_ref()) {
          let size = window.inner_size();
          let w = size.width.max(1);
          let h = size.height.max(1);
          if self.flag {
            object_state.resize((w, h).into());
          } else {
            object_state.resize((w / 2, h / 2).into());
          }
          self.flag = !self.flag;
          window.request_redraw();
        }
      }
      WindowEvent::RedrawRequested => {
        if let Some(object_state) = self.object_state.as_mut() {
          let _ = object_state.render();
        }
      }
      _ => (),
    }
  }
}
