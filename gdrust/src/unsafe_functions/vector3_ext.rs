use gdnative::prelude::Vector3;

pub trait Vector3Ext {
    /// Returns the UP unit vector: (0, 1, 0).
    fn up() -> Vector3;

    /// Returns the FORWARD unit vector: (0, 0, 1).
    fn forward() -> Vector3;

    /// Returns the RIGHT unit vector: (1, 0, 0).
    fn right() -> Vector3;
}

impl Vector3Ext for Vector3 {
    #[inline]
    fn up() -> Vector3 {
        Vector3::new(0.0, 1.0, 0.0)
    }

    #[inline]
    fn forward() -> Vector3 {
        Vector3::new(0.0, 0.0, 1.0)
    }

    #[inline]
    fn right() -> Vector3 {
        Vector3::new(1.0, 0.0, 0.0)
    }
}
