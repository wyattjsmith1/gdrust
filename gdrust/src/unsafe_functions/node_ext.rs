use crate::unsafe_functions::option_ext::OptionExt;
use gdnative::prelude::{Node, NodePath, Shared, SubClass};
use gdnative::NewRef;
use gdnative::TRef;

pub trait NodeExt {
    /// Gets a typed node from a node path. This has an explicit `unsafe` block, and can panic. The
    /// unsafe code is calling `assume_safe` on the node at `path`.
    /// # Panics
    /// - If no node is found at the path.
    /// - If a node is found at the path, but is not the correct type.
    fn require_typed_node<T: SubClass<Node>, P: Into<NodePath>>(&self, path: P) -> TRef<T>;

    /// Gets the parent node with a type. This has an explicit `unsafe` block, and can panic.The
    /// unsafe code is calling `assume_safe` on the parent node.
    /// # Panics
    /// - If no parent is found (root node).
    /// - If a node is found at the path, but is not the correct type.
    fn parent_as<T: SubClass<Node>>(&self) -> TRef<T>;
}

impl<'a, T: SubClass<Node>> NodeExt for TRef<'a, T> {
    fn require_typed_node<Child: SubClass<Node>, P: Into<NodePath>>(
        &self,
        path: P,
    ) -> TRef<'a, Child, Shared> {
        let path = path.into();
        unsafe {
            self.upcast()
                .get_node(path.new_ref())
                .godot_expect(
                    format!("Could not find a node at {}", path.new_ref().to_string()).as_str(),
                )
                .assume_safe()
                .cast::<Child>()
                .godot_expect("Could not cast")
        }
    }

    fn parent_as<Child: SubClass<Node>>(&self) -> TRef<'a, Child, Shared> {
        unsafe {
            self.upcast()
                .get_parent()
                .godot_expect("Could not get a parent node")
                .assume_safe()
                .cast::<Child>()
                .godot_expect("Could not cast")
        }
    }
}
