#[cfg(test)]
mod tests {
  use charbs::events::*;

  #[derive(Event, Clone, Debug, PartialEq)]
  struct CustomEvent1 {
    message: String,
  }

  #[derive(Event, Clone, Debug, PartialEq)]
  struct CustomEvent2 {
    data: u32,
  }

  #[test]
  fn event_bus_works() {
    let event = CustomEvent1 {
      message: "event1".to_string(),
    };

    let mut event_bus = EventBus::default();
    event_bus.write(event.clone());

    let mut test_events = event_bus.read::<CustomEvent1>();

    assert_eq!(test_events.pop(), Some(event));
  }

  #[test]
  fn event_bus_clears_events() {
    let event1 = CustomEvent1 {
      message: "event1".to_string(),
    };
    let event2 = CustomEvent1 {
      message: "event2".to_string(),
    };

    let mut event_bus = EventBus::default();
    event_bus.write(event1);
    event_bus.write(event2);

    event_bus.clear();

    let test_events = event_bus.read::<CustomEvent1>();

    assert_eq!(test_events.len(), 0);
  }

  #[test]
  fn event_bus_multiple_types() {
    let event1 = CustomEvent1 {
      message: "event1".to_string(),
    };
    let event2 = CustomEvent2 { data: 42 };

    let mut event_bus = EventBus::default();
    event_bus.write(event1.clone());
    event_bus.write(event2.clone());

    let mut test_events = event_bus.read::<CustomEvent1>();
    let mut custom_events = event_bus.read::<CustomEvent2>();

    assert_eq!(test_events.pop(), Some(event1));
    assert_eq!(custom_events.pop(), Some(event2));
  }
}
