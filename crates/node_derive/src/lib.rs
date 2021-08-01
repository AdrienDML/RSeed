use proc_macro::TokenStream;
use syn::ItemImpl;
use syn::parse::Parse;
use syn::parse_macro_input;
use syn::ItemStruct;
use syn::Ident;
use syn::Token;
use syn::punctuated::Punctuated;

use quote::quote;

mod fields;

mod node;
mod node2d;
mod node3d;

enum NodeType {
    Node,
    Node3D,
    Node2D,
}

struct Args {
    pub types : Vec<NodeType>,
}

impl NodeType {
    pub fn add_fields_to_struct(&self, in_struct : &mut ItemStruct) {
        match self {
            Self::Node => fields::add_node_fields(in_struct, node::node_struct()),
            Self::Node2D => todo!(),
            Self::Node3D => todo!(),
        }
    }

    pub fn get_impl(&self, struct_ident : Ident) -> ItemImpl {
        match self {
            Self::Node => fields::get_node_impl(struct_ident, node::node_impl()),
            Self::Node2D => todo!(),
            Self::Node3D => todo!(),
        }
    } 

}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let types = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        let mut nt = Vec::new();
        for ntype in types.iter() {
            let s = ntype.to_string();
            match s.as_str() {
                "Node" => nt.push(NodeType::Node),
                "Node2D" => nt.push(NodeType::Node2D),
                "Node3D" => nt.push(NodeType::Node3D),
                _ => return Err(syn::Error::new(ntype.span(), format!("Invalid Node trait : {}.", s)))
            }
        }
        Ok(Self {
            types : nt,
        })
    }
}

fn impl_node_derives(args : Args, in_struct : ItemStruct) -> TokenStream {
    let mut new_struct = in_struct;
    let mut trait_impls : Vec<ItemImpl> = Vec::new();
    for arg in args.types.iter() {
        arg.add_fields_to_struct(&mut new_struct);
        trait_impls.push(arg.get_impl(new_struct.ident.clone()))
    }
    let output = quote!{
        #new_struct
        #(#trait_impls)*
    };
    output.into()
}

#[proc_macro_attribute]
pub fn node_derive(attr: TokenStream, item: TokenStream) -> TokenStream {
    let derives = parse_macro_input!(attr as Args);
    let input = parse_macro_input!(item as ItemStruct);
    impl_node_derives(derives, input)
}





#[cfg(test)]
mod test {

    // #[test]
    // fn correct_attr() {
    //     assert_eq!(NodeTypes::Node, Token![])
    // }
}