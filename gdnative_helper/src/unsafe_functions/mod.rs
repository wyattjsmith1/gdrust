//! A collection of unsafe functions to use. It is highly recommended you understand
//! `gdnative-rust`'s memory model and read the documentation on these methods. These methods are
//! only really unsafe if you don't understand what they are doing.

pub mod node_ext;
pub mod spatial_ext;

pub(crate) fn godot_panic(message: String) -> ! {
    gdnative::godot_error!("{}", message);
    panic!("{}", message)
}
