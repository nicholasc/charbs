extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn impl_schedule_label(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let gen = quote! {
    impl ScheduleLabel for #name {}
  };

  gen.into()
}
