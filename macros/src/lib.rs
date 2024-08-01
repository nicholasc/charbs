extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ScheduleLabel)]
pub fn schedule_label_derive(input: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let input = parse_macro_input!(input as DeriveInput);

  // Build the impl
  impl_schedule_label(&input)
}

fn impl_schedule_label(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let gen = quote! {
    impl ScheduleLabel for #name {}
  };

  gen.into()
}

#[proc_macro_derive(Event)]
pub fn event_derive(input: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let input = parse_macro_input!(input as DeriveInput);

  // Build the impl
  impl_event(&input)
}

fn impl_event(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let gen = quote! {
    impl Event for #name {
      fn as_any(self: Box<Self>) -> Box<dyn Any>
      where
        Self: Sized + 'static,
      {
        self
      }
    }
  };

  gen.into()
}
