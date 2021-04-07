use crate::unsafe_functions::result_ext::ResultExt;
use gdnative::prelude::{Node, PackedScene, SubClass};
use gdnative::TRef;

pub trait PackedSceneExt<'a> {
    /// Tries to instance a scene (with `edit_state=0)` and cast it. Returns errors if it fails.
    ///
    /// # Errors
    /// `FailedToInstance`: If `ResourceLoader.instance(0)` returns `None`.
    /// `IncorrectType`: If the cast to `T` fails.
    fn try_instance_as<T: SubClass<Node>>(&'a self) -> Result<TRef<'a, T>, InstanceSceneErr>;

    /// Expects a scene to be instanced correctly. Same as `try_instance_as` with an expect.
    fn expect_instance_as<T: SubClass<Node>>(&'a self) -> TRef<'a, T>;
}

impl<'a> PackedSceneExt<'a> for TRef<'a, PackedScene> {
    fn try_instance_as<T: SubClass<Node>>(&'a self) -> Result<TRef<'a, T>, InstanceSceneErr> {
        self.instance(0)
            .ok_or(InstanceSceneErr::FailedToInstance)
            .and_then(|x| unsafe {
                x.assume_safe()
                    .cast::<T>()
                    .ok_or(InstanceSceneErr::IncorrectType)
            })
    }

    fn expect_instance_as<T: SubClass<Node>>(&'a self) -> TRef<'a, T> {
        self.try_instance_as::<T>()
            .godot_expect("Failed to instance scene")
    }
}

#[derive(Debug)]
pub enum InstanceSceneErr {
    FailedToInstance,
    IncorrectType,
}
