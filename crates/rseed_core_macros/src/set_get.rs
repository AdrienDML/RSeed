use proc_macro::TokenStream;
use syn::{Ident, Token, Type, TypeInfer, parse::{Parse, ParseStream}, token::{Colon, Underscore}};
use quote::{quote, format_ident};

pub struct Args {
    field : Ident,
    ty : Type,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let field = input.parse::<Ident>()?;
        input.parse::<Colon>()?;
        let ty = input.parse()?;
        Ok(Self {
            field,
            ty,
        })
    }
}


pub fn set(args : &Args) -> TokenStream {
    let fct_ident = format_ident!("set_{}", args.field);
    let Args{field, ty} = args;
    let out = quote!{
        pub fn #fct_ident(&mut self, #field : #ty) {
            self.#field = #field;
        }
    };
    out.into()
}

pub fn pset(args : &Args) -> TokenStream {
    let fct_ident = format_ident!("set_{}", args.field);
    let Args{field, ty} = args;
    let out = quote!{
        pub fn #fct_ident(&mut self,#field : #ty, _ : rseed_core::private::Local) {
            self.#field = #field;
        }
    };
    out.into()
}

pub fn get(args : &Args) -> TokenStream {
    let fct_ident = format_ident!("get_{}", args.field);
    let Args{field, ty} = args;
    let out = quote!{
        pub fn #fct_ident(&self) -> &#ty {
            &self.#field
        }
    };
    out.into()
}