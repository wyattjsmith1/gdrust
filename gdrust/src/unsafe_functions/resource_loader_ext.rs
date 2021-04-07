use crate::unsafe_functions::result_ext::ResultExt;
use gdnative::prelude::{GodotString, PackedScene, ResourceLoader};
use gdnative::Ref;

pub trait ResourceLoaderExt {
    /// Tries to load a scene.
    ///
    /// # Errors
    /// `NoSuchScene`: If there is no scene at that path.
    /// `NotAScene`: If the resource at the path is not a `PackedScene`.
    fn try_load_scene(
        &self,
        path: impl Into<GodotString>,
    ) -> Result<Ref<PackedScene>, LoadSceneErr>;

    /// Expects a scene at the given path and panics if it can not find it. See `try_get_scene` for
    /// a safer alternative. This method just unwraps the result.
    fn expect_load_scene(&self, path: impl Into<GodotString>) -> Ref<PackedScene>;
}

impl ResourceLoaderExt for ResourceLoader {
    fn try_load_scene(
        &self,
        path: impl Into<GodotString>,
    ) -> Result<Ref<PackedScene>, LoadSceneErr> {
        self.load(path.into(), "", false)
            .ok_or_else(|| LoadSceneErr::NoSuchScene)
            .and_then(|x| {
                x.cast::<PackedScene>()
                    .ok_or_else(|| LoadSceneErr::NotAScene)
            })
    }

    fn expect_load_scene(&self, path: impl Into<GodotString>) -> Ref<PackedScene> {
        self.try_load_scene(path)
            .godot_expect("Failed to load scene")
    }
}

#[derive(Debug)]
pub enum LoadSceneErr {
    NoSuchScene,
    NotAScene,
}
