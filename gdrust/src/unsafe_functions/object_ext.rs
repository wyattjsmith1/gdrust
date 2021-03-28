use crate::godot_panic;
use gdnative::prelude::{NativeClass, Object, RefInstance, SubClass};
use gdnative::thread_access::ThreadAccess;
use gdnative::{GodotObject, TRef};

/// An error when using `try_as_instance`.
pub enum TryAsError {
    /// We were unable to get as an instance because the node was not of the correct type.
    Cast,
    /// We were unable to get as an instance because the expected script was not attached to the
    /// Node.
    Instance,
}

pub trait ObjectExt<'a, A: ThreadAccess> {
    /// Tries to cast a given node as `T`. Returns `Ok` with the `RefInstance` if found. Returns `Err`
    /// if it was unable to get the `RefInstance`:
    /// # Errors
    /// `TryAsError::Cast`: If the given node is not the correct type for the script.
    /// `TryAsError::Instance`: If the given node does not have the correct script attached.
    fn try_as_instance<T: NativeClass>(self) -> Result<RefInstance<'a, T, A>, TryAsError>
    where
        <T as NativeClass>::Base: SubClass<gdnative::prelude::Object>;

    /// Expects the passed in node has the `T` script attached. Panics if not. Same as `try_as_instance`
    /// but panics on `Err`
    /// # Panics
    /// If either the given node is not the correct type for the script, or the given node does not have the correct script attached
    fn expect_as_instance<T: NativeClass>(self) -> RefInstance<'a, T, A>
    where
        <T as NativeClass>::Base: SubClass<gdnative::prelude::Object>;
}

impl<'a, A: ThreadAccess> ObjectExt<'a, A> for TRef<'a, Object, A> {
    fn try_as_instance<T: NativeClass>(self) -> Result<RefInstance<'a, T, A>, TryAsError>
    where
        <T as NativeClass>::Base: SubClass<gdnative::prelude::Object>,
    {
        self.cast::<<T as NativeClass>::Base>()
            .ok_or(TryAsError::Cast)
            .and_then(|x| x.cast_instance().ok_or(TryAsError::Instance))
    }

    fn expect_as_instance<T: NativeClass>(self) -> RefInstance<'a, T, A>
    where
        <T as NativeClass>::Base: SubClass<gdnative::prelude::Object>,
    {
        match self.try_as_instance() {
            Ok(x) => x,
            Err(TryAsError::Cast) => godot_panic!(
                "Expected to cast to {}, but that was not found",
                <T as NativeClass>::Base::class_name()
            ),
            Err(TryAsError::Instance) => godot_panic!(
                "Expected Node to have {} attached, but it did not",
                <T as NativeClass>::class_name()
            ),
        }
    }
}
