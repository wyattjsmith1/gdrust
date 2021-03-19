use crate::godot_panic;
use std::fmt::Debug;

pub trait ResultExt<T> {
    /// Performs the exact same functionality as `Result::unwrap`, but prints errors to godot's
    /// output also.
    fn godot_unwrap(self) -> T;

    /// Performs the exact same functionality as `Result::expect`, but prints errors to godot's
    /// output also.
    fn godot_expect(self, msg: &str) -> T;
}

impl<T, E: Debug> ResultExt<T> for Result<T, E> {
    fn godot_unwrap(self) -> T {
        self.unwrap_or_else(|e| {
            godot_panic!("called `Result::unwrap()` on an `Err` value: {:?}", &e)
        })
    }

    fn godot_expect(self, msg: &str) -> T {
        self.unwrap_or_else(|e| godot_panic!("{}: {:?}", msg, &e))
    }
}
