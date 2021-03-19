use crate::godot_panic;

pub trait OptionExt<T> {
    /// Performs the exact same functionality as `Option::unwrap`, but prints errors to godot's
    /// output also.
    fn godot_unwrap(self) -> T;

    /// Performs the exact same functionality as `Option::expect`, but prints errors to godot's
    /// output also.
    fn godot_expect(self, msg: &str) -> T;
}

#[allow(clippy::single_match_else)] // To match Option's syntax
impl<T> OptionExt<T> for Option<T> {
    fn godot_unwrap(self) -> T {
        self.unwrap_or_else(|| godot_panic!("called `Option::unwrap()` on a `None` value"))
    }

    fn godot_expect(self, msg: &str) -> T {
        self.unwrap_or_else(|| godot_panic!("{}", msg))
    }
}
