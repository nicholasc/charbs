use crate::camera::Camera;

use glam::Vec2;

use std::collections::HashMap;

use winit::{
  dpi::PhysicalPosition,
  event::{ElementState, KeyEvent},
  keyboard::{KeyCode, ModifiersState, PhysicalKey},
};

/// A convenient structure to keep track of the current state of the keyboard.
///
/// A default application loop uses this structure when receiving winit events
/// to update the keyboard state. This wrapper structure allows to map keys to
/// different input flow names and provides convenient methods to check when
/// these flow are triggered.
#[derive(Default)]
pub struct KeyboardState {
  /// The used-defined bindings to watch for.
  bindings: HashMap<String, Vec<KeyCode>>,

  /// Various state trackers
  keys: HashMap<KeyCode, KeyEvent>,
  modifiers: ModifiersState,
}

impl KeyboardState {
  /// Updates the internal state of the keyboard keys.
  ///
  /// # Arguments
  ///
  /// * `event` - The [`KeyEvent`] received from winit.
  pub fn update_key(&mut self, event: KeyEvent) {
    if let PhysicalKey::Code(key) = event.physical_key {
      self
        .keys
        .entry(key)
        .and_modify(|e| *e = event.clone())
        .or_insert(event);
    }
  }

  /// Updates the internal state of the keyboard modifiers.
  ///
  /// # Arguments
  ///
  /// * `modifiers` - The [`ModifiersState`] from winit.
  pub fn update_modifiers(&mut self, modifiers: ModifiersState) {
    self.modifiers = modifiers;
  }

  /// Provides a binding to a [`KeyCode`] by associating the key state to the
  /// user-defined action it represents.
  ///
  /// # Arguments
  ///
  /// * `action` - The name of the input flow
  /// * `key` - The winit [`KeyCode`] to bind to this action.
  pub fn bind(&mut self, action: &str, key: KeyCode) {
    match self.bindings.get_mut(action) {
      Some(entry) => {
        if !entry.contains(&key) {
          entry.push(key);
        }
      }
      None => {
        self.bindings.insert(action.into(), vec![key]);
      }
    }
  }

  /// Provides a mapping to multiple [`KeyCode`]s by associating the keys state
  /// to the user-defined action it represents.
  ///
  /// # Arguments
  ///
  /// * `action` - The name of the input flow
  /// * `keys` - An array of winit [`KeyCode`]s to bind to this action.
  pub fn map(&mut self, name: &str, keys: &[KeyCode]) {
    for key in keys {
      self.bind(name, *key);
    }
  }

  /// Reset all keyboard bindings.
  pub fn clear(&mut self) {
    self.bindings.clear();
    self.keys.clear();
    self.modifiers = ModifiersState::default();
  }

  /// Checks if the shift key is pressed.
  pub fn shift(&self) -> bool {
    self.modifiers.shift_key()
  }

  /// Checks if the control key is pressed.
  pub fn control(&self) -> bool {
    self.modifiers.control_key()
  }

  /// Checks if the alt key is pressed.
  pub fn alt(&self) -> bool {
    self.modifiers.alt_key()
  }

  /// Checks if the system key is pressed (super key or command key).
  pub fn system(&self) -> bool {
    self.modifiers.super_key()
  }

  /// Checks if a key that represents an action is currently pressed.
  ///
  /// # Arguments
  ///
  /// * `action` - The action we are checking for.
  ///
  /// * `->` - `true` if the action is currently pressed, `false` otherwise.
  pub fn pressed(&self, action: &str) -> bool {
    let mut pressed = false;

    if let Some(keys) = self.bindings.get(action) {
      for key in keys {
        if let Some(state) = self.keys.get(key) {
          pressed |= state.state == ElementState::Pressed;
        }
      }
    }

    pressed
  }

  /// Check if a key that represents an action is currently pressed in
  /// combination with the shift key.
  ///
  /// # Arguments
  ///
  /// * `action` - The action we are checking for.
  ///
  /// * `->` - `true` if the action is currently pressed with shift, `false`
  ///   otherwise.
  pub fn shift_pressed(&self, action: &str) -> bool {
    self.pressed(action) && self.shift()
  }

  /// Check if a key that represents an action is currently pressed in
  /// combination with the control key.
  ///
  /// # Arguments
  ///
  /// * `action` - The action we are checking for.
  ///
  /// * `->` - `true` if the action is currently pressed with control, `false`.
  pub fn control_pressed(&self, action: &str) -> bool {
    self.pressed(action) && self.control()
  }

  /// Check if a key that represents an action is currently pressed in
  /// combination with the alt key.
  ///
  /// # Arguments
  ///
  /// * `action` - The action we are checking for.
  ///
  /// * `->` - `true` if the action is currently pressed with alt, `false`.
  pub fn alt_pressed(&self, action: &str) -> bool {
    self.pressed(action) && self.alt()
  }

  /// Check if a key that represents an action is currently pressed in
  /// combination with the system key.
  ///
  /// # Arguments
  ///
  /// * `action` - The action we are checking for.
  ///
  /// * `->` - `true` if the action is currently pressed with system, `false`.
  pub fn system_pressed(&self, action: &str) -> bool {
    self.pressed(action) && self.system()
  }
}

#[derive(Default)]
pub struct MouseState {
  window_size: Vec2,
  position: Vec2,
  uv: Vec2,
  world: Vec2,
}

impl MouseState {
  /// Updates the window size based on winit information. Triggers a position
  /// update to make sure that the mouse information is accurate.
  ///
  /// # Arguments
  ///
  /// * `width` - The current window width.
  /// * `height` - The current window height.
  /// * `camera` - The [`Camera`] used to update the mouse world position.
  pub fn update_window_size(&mut self, width: u32, height: u32, camera: &Camera) {
    self.window_size.x = width as f32;
    self.window_size.y = height as f32;

    // Force update the different mouse notations
    self.update_position(
      PhysicalPosition::new(self.position.x as f64, self.position.y as f64),
      camera,
    );
  }

  /// Updates the mouse position based on winit information. Also updates the
  /// UVs and the world position based on the [`Camera`] position.
  ///
  /// # Arguments
  ///
  /// * `position` - The current [`PhysicalPosition`] of the mouse cursor.
  /// * `camera` - The [`Camera`] used to update the mouse world position.
  pub fn update_position(&mut self, position: PhysicalPosition<f64>, camera: &Camera) {
    if !position.x.is_nan() {
      self.position.x = position.x as f32;
    }

    if !position.y.is_nan() {
      self.position.y = position.y as f32;
    }

    // UV
    self.uv.x = self.position.x / self.window_size.x;
    self.uv.y = 1.0 - self.position.y / self.window_size.y;

    // World position
    self.world.x = 2.0 * (-0.5 + self.uv.x) + camera.transform.position.x;
    self.world.y = 2.0 * (-0.5 + self.uv.y) + camera.transform.position.y;
  }

  /// Returns a read-only reference to the current mouse position.
  pub fn position(&self) -> &Vec2 {
    &self.position
  }

  /// Returns a read-only reference to the current mouse uv position.
  pub fn uv(&self) -> &Vec2 {
    &self.uv
  }

  /// Returns a read-only reference to the current mouse world position.
  pub fn world(&self) -> &Vec2 {
    &self.world
  }
}
