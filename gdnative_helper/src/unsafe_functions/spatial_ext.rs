use euclid::approxeq::ApproxEq;
use euclid::{RigidTransform3D, Rotation3D, Trig};
use gdnative::api::Spatial;
use gdnative::object::SubClass;
use gdnative::prelude::Transform;
use gdnative::TRef;
use num_traits::cast::NumCast;
use num_traits::Float;

pub trait SpatialExt {
    fn set_rotation<T: Copy + NumCast + Float + ApproxEq<T> + Trig, Src: Copy, Dst: Copy>(
        &self,
        rotation: Rotation3D<T, Src, Dst>,
    );
}

impl<'a, Class: SubClass<Spatial>> SpatialExt for TRef<'a, Class> {
    fn set_rotation<T: Copy + NumCast + Float + ApproxEq<T> + Trig, Src: Copy, Dst: Copy>(
        &self,
        rotation: Rotation3D<T, Src, Dst>,
    ) {
        let spatial = self.upcast::<Spatial>();
        let origin = spatial.global_transform().origin;
        let rigid_transform = RigidTransform3D {
            rotation: rotation.to_untyped().cast_unit(),
            translation: origin.cast_unit().cast(),
        };
        spatial.set_global_transform(Transform::from_transform(
            &rigid_transform.to_transform().cast(),
        ));
    }
}
