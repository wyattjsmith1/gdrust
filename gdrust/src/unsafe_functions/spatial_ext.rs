use gdnative::api::Spatial;
use gdnative::object::SubClass;
use gdnative::prelude::Basis;
use gdnative::TRef;

pub trait SpatialExt {
    /// Sets the global rotation of the Spatial.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// spatial.global_transform.basis = basis
    /// ```
    fn set_global_rotation(&self, basis: Basis);
}

impl<'a, Class: SubClass<Spatial>> SpatialExt for TRef<'a, Class> {
    fn set_global_rotation(&self, rotation: Basis) {
        let spatial = self.upcast::<Spatial>();
        let mut transform = spatial.global_transform();
        transform.basis = rotation;
        spatial.set_global_transform(transform);
    }
}
