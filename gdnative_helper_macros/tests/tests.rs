use gdnative::prelude::{Ref, Texture};
use gdnative_helper_macros::gdscript;

gdscript! {
    class HelloWorld extends gdnative::api::KinematicBody {
        @export var test_a: u8 = 10
        @no_export var test_b: &'static str = "Test string"
        var test_c: f32 = 10.0

        @export_range(0.0, 10.0) var simple_range: f32 = 0.0
        @export_range(0, 10, 1) var step_range: u8 = 2
        @export_range(0, 10, "or_lesser") var simple_range_or_lesser: u64 = 10
        @export_range(0.0, 10.0, 1.5, "or_lesser") var simple_range_step_or_lesser: f64 = 10.0
        @export_range(0, 10, "or_greater") var simple_range_or_greater: u64 = 10
        @export_range(0, 10, 10, "or_greater") var simple_range_step_or_greater: u64 = 10
        @export_range(0, 10, 10, "or_lesser", "or_greater") var range_with_all: u64 = 10

        @export var texture: Option<Ref<Texture>> = None
    }
}

#[test]
fn test() {
    // No test necessary if we are able to compile.
    assert!(true)
}
