extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn impl_event(ast: &DeriveInput) -> TokenStream {
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
