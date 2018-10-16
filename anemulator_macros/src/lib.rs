#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_hygiene)]
#![feature(proc_macro_quote)]
#![feature(proc_macro_span)]

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{Expr, Ident, Type, Visibility};

struct ChipInfo {
  state_container: Ident,
  step_fn:         Ident,
  
}

impl Parse for ChipInfo {
  fn parse(input: ParseStream) -> Result<Self> {
    input.parse::<Token![enum]>()?;
    let state_container: Ident = input.parse()?;
    input.parse::<Token![;]>()?;
    input.parse::<Token![fn]>()?;
    let step_fn: Ident = input.parse()?;
    input.parse::<Token![;]>()?;
    Ok(ChipInfo{state_container, step_fn})
  }
}


#[proc_macro]
pub fn define_chip(input: TokenStream) -> TokenStream {
  let ChipInfo {state_container, step_fn} = parse_macro_input!(input as ChipInfo);

  let expanded = quote! {
    enum #state_container { }
  };
  TokenStream::from(expanded)
}
