use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod set_get;


/// Implement the setter & getter for the given field.
/// The macro expect the input as such :
/// ```no_run
/// impl Struct {
///     set_get!(field_name : Field_type)
/// }
#[proc_macro]
pub fn set_get(input : TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as set_get::Args);
    let mut impl_ = set_get::set(&args);
    impl_.extend(set_get::get(&args));
    impl_
}

// TODO: Implement the rest of the set_get macros variants.