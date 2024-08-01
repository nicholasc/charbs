pub use charbs_macros::Event;

use std::{
  any::{Any, TypeId},
  collections::HashMap,
};

/// A trait for structures that can be used as events.
///
/// There is a proc macro that provides automatic impl of the `Event` trait for any struct.
pub trait Event {
  fn as_any(self: Box<Self>) -> Box<dyn Any>
  where
    Self: 'static;
}

/// An event bus structure for sending and receiving events.
#[derive(Default)]
pub struct EventBus {
  events: HashMap<TypeId, Vec<Box<dyn Event>>>,
}

impl EventBus {
  /// Write an event to the event bus.
  ///
  /// # Arguments
  ///
  /// * `event` - The event to be written.
  pub fn write<T: Event + 'static>(&mut self, event: T) {
    let type_id = TypeId::of::<T>();
    let events = self.events.entry(type_id).or_default();

    events.push(Box::new(event));
  }

  /// Read events of a specific type from the event bus.
  ///
  /// # Arguments
  ///
  /// * `->` - A vector of events of the specified type.
  pub fn read<T: Event + 'static>(&mut self) -> Vec<T> {
    let type_id = TypeId::of::<T>();
    let mut events = self.events.remove(&type_id).unwrap_or_default();

    events
      .drain(..)
      .map(|event| *event.as_any().downcast().unwrap())
      .collect()
  }

  /// Clear all events from the event bus.
  pub fn clear(&mut self) {
    self.events.clear();
  }
}
