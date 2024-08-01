extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod event;
mod schedule;

#[proc_macro_derive(ScheduleLabel)]
pub fn schedule_label_derive(input: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let input = parse_macro_input!(input as DeriveInput);

  // Build the impl
  crate::schedule::impl_schedule_label(&input)
}

#[proc_macro_derive(Event)]
pub fn event_derive(input: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let input = parse_macro_input!(input as DeriveInput);

  // Build the impl
  crate::event::impl_event(&input)
}
