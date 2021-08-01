use syn::{ItemImpl, ItemStruct, parse_quote};

pub fn node_struct() -> ItemStruct { 
    parse_quote!(
        struct Dummy {
            pos : Vec3D,
        }
    )
}

pub fn node_impl() -> ItemImpl {
    parse_quote!(
        impl TNode3D for Dummy {
            
        }
    )
}