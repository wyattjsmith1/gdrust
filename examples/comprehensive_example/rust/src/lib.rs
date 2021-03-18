use gdnative::api::Texture;
use gdnative::prelude::{Color, InitHandle};
use gdnative::{godot_init, Ref};
use gdnative_helper::gdnative_helper_macros::gdscript;

gdscript! {
    class HelloWorld extends gdnative::api::Node {
        @export var test_a: u8 = 10
        @no_export var test_b: &'static str = "Test string"
        var test_c: f32 = 10.0

        @export_range(0.0, 10.0) var simple_range: f32 = 0.0
        @export_range(0, 10, 2) var step_range: u8 = 2
        @export_range(0, 10, "or_lesser") var simple_range_or_lesser: u64 = 10
        @export_range(0.0, 10.0, 1.5, "or_lesser") var simple_range_step_or_lesser: f64 = 10.0
        @export_range(0, 10, "or_greater") var simple_range_or_greater: u64 = 10
        @export_range(0, 10, 10, "or_greater") var simple_range_step_or_greater: u64 = 10
        @export_range(0, 10, 10, "or_lesser", "or_greater") var range_with_all: u64 = 10

        @export var texture: Option<Ref<Texture>> = None

        @export_enum("This", "is", "a", "test") var string_enum: String = "This".to_string()
        @export_enum("This", "will", "be", "enum", "ordinals") var int_enum: u32 = 0

        @export_file var file: String = "".to_string()
        @export_file("*.png") var png_file: String = "".to_string()

        @export_dir var dir: String = "".to_string()
        @export_global_file("*.png") var glob_file: String = "".to_string()
        @export_global_dir var glob_dir: String = "".to_string()

        @export_multiline var multiline: String = "This is multiline text".to_string()

        @export_exp_range(0.0, 10.0) var simple_exp_range: f32 = 0.0
        @export_exp_range(0, 10, 2) var step_exp_range: u8 = 2
        @export_exp_range(0, 10, "or_lesser") var simple_exp_range_or_lesser: u64 = 10
        @export_exp_range(0.0, 10.0, 1.5, "or_lesser") var simple_exp_range_step_or_lesser: f64 = 10.0
        @export_exp_range(0, 10, "or_greater") var simple_exp_range_or_greater: u64 = 10
        @export_exp_range(0, 10, 10, "or_greater") var simple_exp_range_step_or_greater: u64 = 10
        @export_exp_range(0, 10, 10, "or_lesser", "or_greater") var exp_range_with_all: u64 = 10

        @export var color: Color = Color::rgba(0.0, 0.0, 0.0, 0.5)
        @export_color_no_alpha var color_no_alpha: Color = Color::rgb(0.0, 0.0, 0.0)
    }
}

fn init(handle: InitHandle) {
    handle.add_tool_class::<HelloWorld>();
}

godot_init!(init);
