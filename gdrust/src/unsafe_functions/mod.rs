//! A collection of unsafe functions to use. It is highly recommended you understand
//! `gdnative-rust`'s memory model and read the documentation on these methods. These methods are
//! only really unsafe if you don't understand what they are doing.

pub mod node2_ext;
pub mod node_ext;
pub mod object_ext;
pub mod option_ext;
pub mod packed_scene_ext;
pub mod resource_loader_ext;
pub mod result_ext;
pub mod spatial_ext;
pub mod vector2_ext;
pub mod vector3_ext;

/// Same functionality as `panic!()`, but also outputs to the godot output.
#[macro_export]
macro_rules! godot_panic {
    ($($args:tt)*) => {
        {
            gdnative::godot_error!($($args)*);
            panic!($($args)*);
        }
    }
}

/// Same functionality as `assert!()`, but also outputs to the godot output.
#[macro_export]
macro_rules! godot_assert {
    ($condition:expr $(,)?) => {
        if !$condition {
            gdnative::godot_error!("Assertion error: {}", stringify!($condition));
            panic!("Assertion error: {}", stringify!($condition));
        }
    };
    ($condition:expr, $($args:tt)*) => {
        if !$condition {
            gdnative::godot_error!($($args)*);
            panic!($($args)*);
        }
    };
}

/// Same functionality as `debug_assert!()`, but also outputs to the godot output.
#[macro_export]
macro_rules! godot_debug_assert {
    ($condition:expr $(,)?) => {

        if cfg!(debug_assertions) && !$condition {
            gdnative::godot_error!("Assertion error: {}", stringify!($condition));
            panic!("Assertion error: {}", stringify!($condition));
        }
    };
    ($condition:expr, $($args:tt)*) => {
        if cfg!(debug_assertions) && !$condition {
            gdnative::godot_error!($($args)*);
            panic!($($args)*);
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn godot_assert_true() {
        godot_assert!(true)
    }

    #[test]
    fn godot_assert_message_true() {
        godot_assert!(true, "this should not {}", "happen")
    }

    #[test]
    fn godot_debug_assert_true() {
        godot_debug_assert!(true)
    }

    #[test]
    fn godot_debug_assert_message_true() {
        godot_debug_assert!(true, "this should not {}", "happen")
    }
}
