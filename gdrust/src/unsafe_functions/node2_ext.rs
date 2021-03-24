use crate::unsafe_functions::vector2_ext::Vector2Ext;
use gdnative::api::Node2D;
use gdnative::object::SubClass;
use gdnative::prelude::{Angle, Vector2, Vector2Godot};
use gdnative::TRef;

pub trait Node2DExt {
    /// Returns the unit vector representing up relative to the `Node2D`'s global orientation.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// Node2D.global_transform.xform(Vector2(0, 1))
    /// ```
    fn global_up(&self) -> Vector2;

    /// Returns the unit vector representing up relative to the `Node2D`'s local orientation.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// Node2D.transform.xform(Vector2(0, 1))
    /// ```
    fn up(&self) -> Vector2;

    /// Returns the unit vector representing right relative to the `Node2D`'s global orientation.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// Node2D.global_transform.xform(Vector2(1, 0))
    /// ```
    fn global_right(&self) -> Vector2;

    /// Returns the unit vector representing right relative to the `Node2D`'s local orientation.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// Node2D.transform.xform(Vector2(1, 0))
    /// ```
    fn right(&self) -> Vector2;
}

impl<'a, Class: SubClass<Node2D>> Node2DExt for TRef<'a, Class> {
    fn global_up(&self) -> Vector2 {
        Vector2::up().rotated(Angle::radians(self.upcast::<Node2D>().rotation() as f32))
    }

    fn up(&self) -> Vector2 {
        Vector2::up().rotated(Angle::radians(
            self.upcast::<Node2D>().global_rotation() as f32
        ))
    }

    fn global_right(&self) -> Vector2 {
        Vector2::right().rotated(Angle::radians(self.upcast::<Node2D>().rotation() as f32))
    }

    fn right(&self) -> Vector2 {
        Vector2::right().rotated(Angle::radians(
            self.upcast::<Node2D>().global_rotation() as f32
        ))
    }
}
