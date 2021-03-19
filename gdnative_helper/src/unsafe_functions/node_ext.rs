use crate::unsafe_functions::godot_panic;
use gdnative::prelude::{Node, NodePath, Shared, SubClass};
use gdnative::NewRef;
use gdnative::TRef;

pub trait NodeExt {
    /// Gets a typed node from a node path. This has an explicit `unsafe` block, and can panic.
    /// # Panics
    /// - If no node is found at the path.
    /// - If a node is found at the path, but is not the correct type.
    fn require_typed_node<T: SubClass<Node>, P: Into<NodePath>>(&self, path: P) -> TRef<T>;

    /// Gets the parent node with a type. This has an explicit `unsafe` block, and can panic.
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
                .unwrap_or_else(|| {
                    godot_panic(
                        format!("Could not find a node at {}", path.new_ref().to_string()).as_str(),
                    )
                })
                .assume_safe()
                .cast::<Child>()
                .expect("Could not cast")
        }
    }

    fn parent_as<Child: SubClass<Node>>(&self) -> TRef<'a, Child, Shared> {
        unsafe {
            self.upcast()
                .get_parent()
                .unwrap_or_else(|| godot_panic("Could not get a parent node"))
                .assume_safe()
                .cast::<Child>()
                .expect("Could not cast")
        }
    }
}
