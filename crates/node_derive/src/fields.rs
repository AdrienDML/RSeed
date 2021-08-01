use syn::{Field, Fields, Ident, ItemImpl, ItemStruct, Path, PathArguments, PathSegment, Type, TypePath, punctuated::Punctuated};

/// A trait that simplify adding fields to a struct.
pub trait AddField {
    fn add_field(&mut self, field : Field);
}

impl AddField for ItemStruct {
    fn add_field(&mut self, field : Field) {
        if let Fields::Named(ref mut fields) = self.fields {
            fields.named.push(field);
        }
    }
}

/// Add the fields from the dummy struct to the in_struct.
pub fn add_node_fields(in_struct : &mut ItemStruct, dummy_struct : ItemStruct) {
    dummy_struct.fields.iter().for_each(|f| {
        in_struct.add_field(f.clone());
    })
}

// Implements the trait for the struct identifient passed in.
pub fn get_node_impl(struct_ident : Ident, dummy_trait_impl : ItemImpl) -> ItemImpl {
    let mut trait_impl = dummy_trait_impl.clone();
    let mut struct_ty = Punctuated::new();
    struct_ty.push_value(PathSegment{ ident : struct_ident, arguments : PathArguments::None});
    trait_impl.self_ty = Box::new(
        Type::Path(
            TypePath {
                qself : None,
                path : Path {
                    leading_colon : None,
                    segments : struct_ty,
                }
            }
        )
    );
    trait_impl
}