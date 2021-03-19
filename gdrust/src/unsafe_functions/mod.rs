#![macro_use]

//! A collection of unsafe functions to use. It is highly recommended you understand
//! `gdnative-rust`'s memory model and read the documentation on these methods. These methods are
//! only really unsafe if you don't understand what they are doing.

pub mod node_ext;
pub mod option_ext;
pub mod result_ext;
pub mod spatial_ext;

#[macro_export]
macro_rules! godot_panic {
    ($($args:tt)*) => {
        {
            gdnative::godot_error!($($args)*);
            panic!($($args)*);
        }
    }
}
