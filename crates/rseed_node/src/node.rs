// pub enum NodeError {
//     IndexOOB
// }

// pub trait TNode {
//     /// Set the name of the node.
//     fn set_name<'a>(&mut self, name : String);
//     /// Get the number of children the node has.
//     fn get_child_count(&self) -> usize;
//     /// Get the child situated at the index. The first child is at index 0. 
//     fn get_child(&self, ind : usize) -> Result<&Box<dyn TNode>, NodeError>;
//     /// Add a child as the last one.
//     fn add_child(&mut self, child : Box<dyn TNode>);
//     /// Add a child at a certain index.
//     fn add_child_at_idx(&mut self, child : Box<dyn TNode>, idx : usize) ->Result<(), NodeError>;
//     /// Remove the provided child
//     fn remove_child(&mut self, child : Box<dyn TNode>);
//     /// Move the child at the provided index.
//     fn move_child(&self, child : Box<dyn TNode>, idx : usize);
//     /// Returns the parent Node if it has one, else return `None`
//     fn get_parent(&self) -> &Option<Box<dyn TNode>>;
//     /// Return `True` if it is the child of a Node.
//     fn is_child(&self) -> bool {self.get_parent().is_some()}
//     /// Return `True if it has no parent.
//     fn is_root_node(&self) -> bool {self.get_parent().is_none()}

//     fn hide(&self, )
// }

// pub struct Node {
//     name : String,
//     parent : Option<Box<dyn TNode>>, /// The parent Node
//     pos : Option<usize>, /// the position of the node in the parent's childrens
//     childrens : Vec<Box<dyn TNode>>,
//     visible : bool,
// }

// impl Node {
//     pub fn new(name : String, parent : Option<Box<dyn TNode>>) -> Node {
//         Self {
//             name,
//             parent,
//             childrens : Vec::new(),
//         }
//     }
// }


