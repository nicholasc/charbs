extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ScheduleLabel)]
pub fn default_partial_eq_eq_derive(input: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let input = parse_macro_input!(input as DeriveInput);

  // Build the impl
  impl_default_partial_eq_eq(&input)
}

fn impl_default_partial_eq_eq(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let gen = quote! {
    impl ScheduleLabel for #name {}
  };

  gen.into()
}
