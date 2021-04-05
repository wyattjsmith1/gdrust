use crate::unsafe_functions::vector3_ext::Vector3Ext;
use gdnative::api::Spatial;
use gdnative::object::SubClass;
use gdnative::prelude::{Basis, Vector3};
use gdnative::TRef;

pub trait SpatialExt {
    /// Sets the local rotation of the Spatial.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// spatial.transform.basis = basis
    /// ```
    fn set_local_rotation(&self, basis: Basis);

    /// Sets the local origin of the Spatial.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// spatial.transform.origin = origin
    /// ```
    fn set_local_origin(&self, origin: Vector3);

    /// Sets the global rotation of the Spatial.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// spatial.global_transform.basis = basis
    /// ```
    fn set_global_rotation(&self, basis: Basis);

    /// Sets the global origin of the Spatial.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// spatial.global_transform.origin = origin
    /// ```
    fn set_global_origin(&self, origin: Vector3);

    /// Returns the unit vector representing the direction the `Spatial` is facing. Godot has some
    /// [ambiguity](https://github.com/godotengine/godot/issues/15283) regarding the forward vector,
    /// so we are using (0, 1, 0), or Z+ forward.
    /// # GdScript Equivalent
    /// ```gdscript
    /// spatial.global_transform.xform(Vector3(0, 0, 1))
    /// ```
    fn global_forward(&self) -> Vector3;

    /// Returns the unit vector representing the direction the `Spatial` is facing. Godot has some
    /// [ambiguity](https://github.com/godotengine/godot/issues/15283) regarding the forward vector,
    /// so we are using (0, 1, 0), or Z+ forward.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// spatial.transform.xform(Vector3(0, 0, 1))
    /// ```
    fn forward(&self) -> Vector3;

    /// Returns the unit vector representing up relative to the `Spatial`'s global orientation.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// spatial.global_transform.xform(Vector3(0, 1, 0))
    /// ```
    fn global_up(&self) -> Vector3;

    /// Returns the unit vector representing up relative to the `Spatial`'s local orientation.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// spatial.transform.xform(Vector3(0, 1, 0))
    /// ```
    fn up(&self) -> Vector3;

    /// Returns the unit vector representing right relative to the `Spatial`'s global orientation.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// spatial.global_transform.xform(Vector3(1, 0, 0))
    /// ```
    fn global_right(&self) -> Vector3;

    /// Returns the unit vector representing right relative to the `Spatial`'s local orientation.
    ///
    /// # GdScript Equivalent
    /// ```gdscript
    /// spatial.transform.xform(Vector3(1, 0, 0))
    /// ```
    fn right(&self) -> Vector3;
}

impl<'a, Class: SubClass<Spatial>> SpatialExt for TRef<'a, Class> {
    fn set_local_rotation(&self, basis: Basis) {
        let spatial = self.upcast::<Spatial>();
        let mut transform = spatial.transform();
        transform.basis = basis;
        spatial.set_transform(transform);
    }

    fn set_local_origin(&self, origin: Vector3) {
        let spatial = self.upcast::<Spatial>();
        let mut transform = spatial.transform();
        transform.origin = origin;
        spatial.set_transform(transform);
    }

    fn set_global_rotation(&self, basis: Basis) {
        let spatial = self.upcast::<Spatial>();
        let mut transform = spatial.global_transform();
        transform.basis = basis;
        spatial.set_global_transform(transform);
    }

    fn set_global_origin(&self, origin: Vector3) {
        let spatial = self.upcast::<Spatial>();
        let mut transform = spatial.global_transform();
        transform.origin = origin;
        spatial.set_global_transform(transform);
    }

    fn global_forward(&self) -> Vector3 {
        self.upcast::<Spatial>()
            .global_transform()
            .basis
            .xform(Vector3::forward())
    }

    fn forward(&self) -> Vector3 {
        self.upcast::<Spatial>()
            .transform()
            .basis
            .xform(Vector3::forward())
    }

    fn global_up(&self) -> Vector3 {
        self.upcast::<Spatial>()
            .global_transform()
            .basis
            .xform(Vector3::up())
    }

    fn up(&self) -> Vector3 {
        self.upcast::<Spatial>()
            .transform()
            .basis
            .xform(Vector3::up())
    }

    fn global_right(&self) -> Vector3 {
        self.upcast::<Spatial>()
            .global_transform()
            .basis
            .xform(Vector3::right())
    }

    fn right(&self) -> Vector3 {
        self.upcast::<Spatial>()
            .transform()
            .basis
            .xform(Vector3::right())
    }
}
