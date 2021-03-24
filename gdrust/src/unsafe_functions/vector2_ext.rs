use gdnative::prelude::Vector2;

pub trait Vector2Ext {
    /// Returns the UP unit vector: (0, 1).
    fn up() -> Vector2;

    /// Returns the RIGHT unit vector: (1, 0)
    fn right() -> Vector2;
}

impl Vector2Ext for Vector2 {
    #[inline]
    fn up() -> Vector2 {
        Vector2::new(0.0, 1.0)
    }

    #[inline]
    fn right() -> Vector2 {
        Vector2::new(1.0, 0.0)
    }
}
