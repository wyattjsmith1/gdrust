use gdnative::api::{Node, Texture};
use gdnative::prelude::{Color, InitHandle, NodePath, ToVariant};
use gdnative::{godot_init, Ref, TRef};
use gdrust::macros::gdrust;

#[gdrust(extends = Node)]
#[signal(my_signal(arg1: F64, arg2: GodotString = "test".to_string()))]
#[signal(simple_signal(arg:I64))]
#[derive(Debug)]
struct HelloWorld {
    
    #[export]
    #[default(10)]
    test_a: u8,

    #[no_export]
    test_failure: u8,

    #[default(10.0)]
    test_c: f32,

    #[export]
    #[set(Self::set_test)]
    test_set: bool,
    
    #[export]
    #[get(Self::get_test)]
    test_get: bool,

    #[export]
    #[setget(Self::set_test, Self::get_test)]
    test_setget: bool,

    #[export_range((-10.0), 10.0)]
    simple_range: f32,

    #[export_range(0, 10, 2)]
    #[default(2)]
    step_range: u8,

    #[export_range(0, 10, "or_lesser")]
    #[default(10)]
    simple_range_or_lesser: i32,

    #[export_range(10.0, 10.0, 1.5, "or_lesser")]
    #[default(-10.0)]
    simple_range_step_or_lesser: f64,

    #[export_range(0, 10, "or_greater")]
    #[default(10)]
    simple_range_or_greater: u64,

    #[export_range(0, 10, 10, "or_greater")]
    #[default(10)]
    simple_range_step_or_greater: u64,

    #[export_range(0, 10, 10, "or_lesser", "or_greater")]
    #[default(10)]
    range_with_all: u64,

    #[export]
    texture: Option<Ref<Texture>>,

    #[export_enum("This", "is", "a", "test")]
    #[default("This".to_string())]
    string_enum: String,

    #[export_enum("This", "will", "be", "enum", "ordinals")]
    int_enum: u32,

    #[export_file]
    file: String,

    #[export_file("*.png")]
    png_file: String,

    #[export_dir]
    dir: String,

    #[export_global_file("*.png")]
    glob_file: String,

    #[export_global_dir]
    glob_dir: String,

    #[export_multiline]
    #[default("This\nis\nmultiline\ntext".to_string())]
    multiline: String,

    #[export_exp_range(0.0, 10.0)]
    simple_exp_range: f32,

    #[export_exp_range(0, 10, 2)]
    #[default(2)]
    step_exp_range: u8,

    #[export_exp_range(0, 10, "or_lesser")]
    #[default(10)]
    simple_exp_range_or_lesser: u64,

    #[export_exp_range(0.0, 10.0, 1.5, "or_lesser")]
    #[default(10.0)]
    simple_exp_range_step_or_lesser: f64,

    #[export_exp_range(0, 10, "or_greater")]
    #[default(10)]
    simple_exp_range_or_greater: u64,

    #[export_exp_range(0, 10, 10, "or_greater")]
    #[default(10)]
    simple_exp_range_step_or_greater: u64,

    #[export_exp_range(0, 10, 10, "or_lesser", "or_greater")]
    #[default(10)]
    exp_range_with_all: u64,

    #[export]
    #[default(Color::rgba(0.0, 0.0, 0.0, 0.5))]
    color: Color,

    #[export_color_no_alpha]
    #[default(Color::rgb(0.0, 0.0, 0.0))]
    color_no_alpha: Color,

    #[export_flags("Fire", "Water", "Earth", "Wind")]
    spell_elements: u32,

    // TODO: NodePath types are only supported in 4.0
    #[export_node_path(KinematicBody, RigidBody)]
    physics_body: NodePath,

    #[export_flags_2d_physics]
    layers_2d_physics: u32,

    #[export_flags_2d_render]
    layers_2d_render: u32,

    #[export_flags_3d_physics]
    layers_3d_physics: u32,

    #[export_flags_3d_render]
    layers_3d_render: u32,
}

#[gdnative::methods]
impl HelloWorld {
    #[export]
    fn set_test(&mut self, owner: TRef<Node>, _val: bool){
        gdnative::godot_print!("tested set on {:#?}!", owner.get_path());
    }
    fn get_test(&self, owner: TRef<Node>) -> &bool{
        gdnative::godot_print!("tested get on {:#?}!", owner.get_path());
        &(self.test_get)
    }
    #[export]
    fn _ready(&self, owner: TRef<Node>) {
        gdnative::godot_print!("Hello World!");
        gdnative::godot_dbg!(self);
        owner
            .upcast::<Node>()
            .emit_signal(Self::SIMPLE_SIGNAL, &[0.to_variant()]);
    }
}

fn init(handle: InitHandle) {
    handle.add_tool_class::<HelloWorld>();
}

godot_init!(init);
