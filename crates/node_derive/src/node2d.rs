use syn::{ItemImpl, ItemStruct, parse_quote};

pub fn node_struct() -> ItemStruct { 
    parse_quote!(
        struct Dummy {
            pos : Vec2D,
        }
    )
}

pub fn node_impl() -> ItemImpl {
    parse_quote!(
        impl TNode2D for Dummy {
            
        }
    )
}