use quote::quote;
use syn::{Field, Fields, FieldsNamed, Ident, ItemImpl, ItemStruct, parse_quote};
use syn::parse::Parse;

use crate::fields::AddField;




pub fn node_struct() -> ItemStruct { 
    parse_quote!(
        struct Dummy {
            name : String,
            parent : Option<Box<dyn TNode>>, 
            pos : Option<usize>, 
            depth : usize,
            childrens : Vec<Box<dyn TNode>>,
            visible : bool,
        }
    )
}



pub fn node_impl() -> ItemImpl {
    parse_quote!(
        impl TNode for Dummy {
            /// Set the name of the node.
            fn set_name<'a>(&mut self, name : String) {
                self.name = name;
            }
            /// Get the number of children the node has.
            fn get_child_count(&self) -> usize {
                self.children.len()
            }
            /// Get the child situated at the index. The first child is at index 0. 
            fn get_child(&self, idx : usize) -> Result<&Box<dyn TNode>, NodeError> {
                self.children.get(idx);
                todo!()
            }
            /// Add a child as the last one.
            fn add_child(&mut self, child : Box<dyn TNode>) {
                self.childrens.push(child)
            }
            /// Add a child at a certain index.
            fn add_child_at_idx(&mut self, child : Box<dyn TNode>, idx : usize) -> Result<(), NodeError>;
            /// Remove the provided child
            fn remove_child(&mut self, child : Box<dyn TNode>);
            /// Move the child at the provided index.
            fn move_child(&self, child : Box<dyn TNode>, idx : usize);
            /// Returns the parent Node if it has one, else return `None`
            fn get_parent(&self) -> &Option<Box<dyn TNode>>;
            /// Return `True` if it is the child of a Node.
            fn is_child(&self) -> bool {self.get_parent().is_some()}
            /// Return `True if it has no parent.
            fn is_root_node(&self) -> bool {self.get_parent().is_none()}
            /// Set the visibility to false.
            fn hide(&mut self);
            /// Set the visibility to true.
            fn show(&mut self);
        }
    )
}