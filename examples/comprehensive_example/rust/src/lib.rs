use gdnative::api::{Node, Texture};
use gdnative::prelude::{Color, InitHandle, NodePath};
use gdnative::{godot_init, Ref, TRef};
use gdnative_helper::gdnative_helper_macros::gdrust;

gdrust! {
    #[derive(Debug)]
    class HelloWorld extends Node {

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

        @export_flags("Fire", "Water", "Earth", "Wind") var spell_elements: u32 = 0

        // TODO: NodePath types are only supported in 4.0
        @export_node_path(KinematicBody, RigidBody) var physics_body: NodePath = NodePath::default()

        signal my_signal(int: I64, float: F64, tex: Texture)
        signal typed_signal(bool: Bool = true, float: F64 = std::f64::consts::PI, tex: Texture)

        @export_flags_2d_physics var layers_2d_physics: u32 = 0
        @export_flags_2d_render var layers_2d_render: u32 = 0
        @export_flags_3d_physics var layers_3d_physics: u32 = 0
        @export_flags_3d_render var layers_3d_render: u32 = 0
    }
}

#[gdnative::methods]
impl HelloWorld {
    #[export]
    fn _ready(&self, _owner: TRef<Node>) {
        gdnative::godot_print!("Hello World!");
        gdnative::godot_dbg!(self);
    }
}

fn init(handle: InitHandle) {
    handle.add_tool_class::<HelloWorld>();
}

godot_init!(init);
