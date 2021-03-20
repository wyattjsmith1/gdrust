#![allow(clippy::doc_markdown)]
//! [![Rust](https://github.com/wyattjsmith1/gdrust/actions/workflows/rust.yml/badge.svg?branch=master&event=push)](https://github.com/wyattjsmith1/gdrust/actions/workflows/rust.yml)
//!
//! A library for making [`gdnative-rust`](https://github.com/godot-rust/godot-rust) a bit more
//! GdScript-like. This contains two main parts:
//!
//! 1. A `gdrust!` macro for simplifying some rust code and making it more GdScript-like.
//! 2. A set of "unsafe" functions to make things more concise at the risk of crashing.
//!
//! # Goals
//! Ultimately, the goal of this project is rust development for Godot more concise in 90% of cases. There may
//! be some edge cases only "true" rust can resolve, and this project should not comprimise its
//! simplicity for the sake of covering every case.
//!
//! # Current State
//! Right now, this project is in an early alpha state. The documented parts should work as expected,
//! but the api is likely to change.
//!
//! # Getting Started
//! `gdrust` surfs on [`gdnative-rust`](https://github.com/godot-rust/godot-rust), so you must have
//! [`gdnative-rust`](https://github.com/godot-rust/godot-rust) setup before you start looking at
//! `gdrust`. Follow their [Getting Started Guide](https://godot-rust.github.io/#installation).
//!
//! Once `gdnative-rust` is installed, you can install `gdrust` by adding it as a dependency.
//! Unfortunately, due to the way `gdnative-rust` macros work, you must have both `gdnative-rust`
//! and `gdrust` added as dependencies, and you must choose compatible versions. See the
//! "Compatibilty" section below.
//! ```ignore
//! [dependencies]
//! gdnative = "0.9.3"
//! gdrust = { git = "https://github.com/wyattjsmith1/gdrust.git" }
//! ```
//!
//! Once installed, simply use the `gdrust` macro:
//! ```
//! use gdrust::macros::gdrust;
//! use gdnative::api::Node;
//!
//! gdrust! {
//!     class_name HelloWorld extends Node
//!     @export var test: u64 = 10
//! }
//! ```
//! That's it!
//!
//! Read more below for details and gotchas with exporting properties and signals, as well as an
//! in-depth comprehensive example.
//!
//! # `gdrust!` Macro
//!
//! ## Exporting "`class`es"
//! Rust doesn't have the concept of a "`class`", but Godot does. To make things a bit more GdScript
//! friendly, regular class notation is used:
//! ```
//! gdrust::macros::gdrust! {
//!     class_name ClassName extends gdnative::api::KinematicBody
//!     // Same as `class_name ClassName extends KinematicBody` in GdScript.
//! }
//! ```
//!
//! The `extends Parent` is optional, and may be omitted if you are just extending `Object`:
//! ```
//! gdrust::macros::gdrust! {
//!     class_name ClassName extends gdnative::api::KinematicBody
//!     // Same as `class_name ClassName extends Object` in GdScript.
//! }
//! ```
//!
//! You can still have custom derives and attributes on your class. Any attributes on `class` will
//! be added:
//! ```
//! gdrust::macros::gdrust! {
//!     #[derive(Debug)]
//!     class_name ClassName extends gdnative::api::KinematicBody
//!     // `ClassName` will derive `Debug`
//! }
//! ```
//!
//! After you create the class and export properties and signals, create your `impl` block as
//! usual. Note, you should not create the `new` function. That is provided by the macro:
//! ```
//!# use gdnative::prelude::*;
//!# use gdnative::api::*;
//!# gdrust::macros::gdrust! {
//!#     #[derive(Debug)]
//!#     class_name ClassName extends gdnative::api::KinematicBody
//!# }
//! #[gdnative::methods]
//! impl ClassName {
//!     #[export]
//!     fn _ready(&self, _owner: TRef<KinematicBody>) {
//!         gdnative::godot_print!("Hello World!")
//!     }
//! }
//! ```
//!
//! ## Exporting Properties
//! The syntax for exporting properties is intended to mirror GdScript as closely as possible. Due
//! to the upcoming 4.0 release, `gdrust` uses the [4.0 syntax](https://docs.godotengine.org/en/latest/tutorials/scripting/gdscript/gdscript_exports.html).
//! You can read all about the different types of exports there. Everything should be implemented as
//! defined, except for the following:
//!
//! 1. `@no_export` can be used to not export a variable. This should be used for all Rust-native
//! types (doesn't implement `Export`) or if you want the variable to be "private".
//! 2. The 4.0 docs define `@export_node_path(Type1, Type2)` as a way to export a `NodePath` which
//! only matches nodes with given types. This is partially implemented, but won't be finished until
//! 4.0 because there is currently not export hint for NodePaths. You can currently include this
//! export in your code, but it will allow a `NodePath` to any type.
//! 3. Nullability is handled with an `Option`. This is required if you don't want to set a defualt
//! value for a type (the default will be `None`).
//! 4. Every `var` (exported or `@no_export`ed) will require both a type and a default value. There
//! is no type inference. In the future, you may be able to use `Default` in place of setting a
//! default. If you are referencing a Godot object and not a "primitive", this must be wrapped in a
//! `Ref`.
//! 5. Currently, arrays are not supported. This is simply because I am not confident the syntax
//! has been finalized. On Godot's site, it shows the traditional `export(Array, int) var ints = [1, 2, 3]`.
//! I am guessing they will switch to some sort of `@export_array` style. Once that is finalized,
//! adding it should be easy.
//!
//! ## Exporting Signals
//! The syntax for exporting signals is also intended to mirror [GdScript](https://docs.godotengine.org/en/latest/getting_started/step_by_step/signals.html#custom-signals)
//! as closely as possible. Similar to properties, there are a few gotchas with signals:
//!
//! 1. Like properties, every signal must have a type. Unlike properties, the type must be one of:
//!   - A [`VariantType`](https://docs.rs/gdnative/0.9.3/gdnative/core_types/enum.VariantType.html)
//!   - A Godot object without a `Ref` (like a `KinematicBody`).
//!
//!   I know this is a little weird, and I'd like to smooth it out a bit. Suggestions are welcome.
//!
//! 2. Unlike GdScript, `gdrust` signal arguments may have optional default values.
//!
//! ## Comprehensive Example
//! This example should contain all possibilities for exporting properties and signals. It is used
//! for testing as well.
//! ```no_run
//!use gdnative::api::{KinematicBody, Node, RigidBody, Texture};
//!use gdnative::prelude::{Color, InitHandle, NodePath};
//!use gdnative::{godot_init, Ref, TRef};
//!use gdrust::macros::gdrust;
//!
//!gdrust! {
//!    #[derive(Debug)]
//!    class_name HelloWorld extends Node
//!    @export var test_a: u8 = 10
//!    @no_export var test_b: &'static str = "Test string"
//!    var test_c: f32 = 10.0
//!    @export_range(0.0, 10.0) var simple_range: f32 = 0.0
//!    @export_range(0, 10, 2) var step_range: u8 = 2
//!    @export_range(0, 10, "or_lesser") var simple_range_or_lesser: u64 = 10
//!    @export_range(0.0, 10.0, 1.5, "or_lesser") var simple_range_step_or_lesser: f64 = 10.0
//!    @export_range(0, 10, "or_greater") var simple_range_or_greater: u64 = 10
//!    @export_range(0, 10, 10, "or_greater") var simple_range_step_or_greater: u64 = 10
//!    @export_range(0, 10, 10, "or_lesser", "or_greater") var range_with_all: u64 = 10
//!    @export var texture: Option<Ref<Texture>> = None
//!    @export_enum("This", "is", "a", "test") var string_enum: String = "This".to_string()
//!    @export_enum("This", "will", "be", "enum", "ordinals") var int_enum: u32 = 0
//!
//!    @export_file var file: String = "".to_string()
//!    @export_file("*.png") var png_file: String = "".to_string()
//!
//!    @export_dir var dir: String = "".to_string()
//!    @export_global_file("*.png") var glob_file: String = "".to_string()
//!    @export_global_dir var glob_dir: String = "".to_string()
//!
//!    @export_multiline var multiline: String = "This is multiline text".to_string()
//!
//!    @export_exp_range(0.0, 10.0) var simple_exp_range: f32 = 0.0
//!    @export_exp_range(0, 10, 2) var step_exp_range: u8 = 2
//!    @export_exp_range(0, 10, "or_lesser") var simple_exp_range_or_lesser: u64 = 10
//!    @export_exp_range(0.0, 10.0, 1.5, "or_lesser") var simple_exp_range_step_or_lesser: f64 = 10.0
//!    @export_exp_range(0, 10, "or_greater") var simple_exp_range_or_greater: u64 = 10
//!    @export_exp_range(0, 10, 10, "or_greater") var simple_exp_range_step_or_greater: u64 = 10
//!    @export_exp_range(0, 10, 10, "or_lesser", "or_greater") var exp_range_with_all: u64 = 10
//!
//!    @export var color: Color = Color::rgba(0.0, 0.0, 0.0, 0.5)
//!    @export_color_no_alpha var color_no_alpha: Color = Color::rgb(0.0, 0.0, 0.0)
//!
//!    @export_flags("Fire", "Water", "Earth", "Wind") var spell_elements: u32 = 0
//!
//!    //TODO: NodePath types are only supported in 4.0
//!    @export_node_path(KinematicBody, RigidBody) var physics_body: NodePath = NodePath::default()
//!
//!    signal my_signal(int: I64, float: F64, tex: Texture)
//!    signal typed_signal(bool: Bool = true, float: F64 = std::f64::consts::PI, tex: Texture)
//!
//!    @export_flags_2d_physics var layers_2d_physics: u32 = 0
//!    @export_flags_2d_render var layers_2d_render: u32 = 0
//!    @export_flags_3d_physics var layers_3d_physics: u32 = 0
//!    @export_flags_3d_render var layers_3d_render: u32 = 0
//!}
//!
//! #[gdnative::methods]
//! impl HelloWorld {
//!     #[export]
//!     fn _ready(&self, _owner: TRef<Node>) {
//!         gdnative::godot_print!("Hello World!")
//!     }
//! }
//! ```
//!
//! ## Pros and Cons
//! Like any piece of software, this is not without it's issues. This list is intended to accurately
//! document the pros and cons to help people decide if this is the right project for them.
//!
//! ### Pros
//!
//! 1. Simplifies the `ClassBuilder` chain and makes the code look more GdScripty
//! 2. Generates a `new`
//! 3. Synchronizes the property default with the `new` default. No more changing the default
//! property value and not having it reflected in code.
//!
//! ### Cons
//! 1. No syntax highlighting or autocomplete. This is by far the biggest issue. Most IDEs don't
//! understand macros as well as we would like. The code in the `gdscript` block, as well as any
//! code it generates, will not have autocompletion or syntax highlighting. Hopefully rust's tools
//! will get better over time to improve this.
//! 2. Like many macros, when the input is correct, they work great. When the input is invalid,
//! they give obscure error messages. I am trying to cover most of the common error cases with clear
//! messages. If you see weird message, open an issue and I will help you out. In general, `@export`s
//! with values require parens (`()`) and you should always use the same type of literals (all ints
//! or all floats).
//! 3. Not 100% gdscript. To meet the needs of Rust this has been designed to look closely like
//! gdscript, but there a couple exceptions.
//!
//! # Unsafe Functions
//! One of the great things about rust is that it forces you to handle every possible case to ensure
//! the runtime goes smoothly. One issue with this is game development is full of "well, I hope this
//! works" cases in which error handling is ignored until runtime.
//!
//! For example, let's say you want to get a child node and call `start_emitting()` on it. In
//! `gdnative-rust`, you would do this:
//! ```ignore
//! unsafe {
//!     owner.get_node("Particles")
//!         .unwrap()
//!         .assume_safe()
//!         .cast::<Particles>()
//!         .unwrap()
//!         .start_emitting();
//! }
//! ```
//! Compare to GdStript (without the $ sugar):
//! ```ignore
//! get_node("Particles").start_emitting()
//! ```
//! Yes, the static typing does cause some verbosity, but this is still a lot. `gdrust` exposes a
//! cleaner method:
//! ```ignore
//! owner.require_typed_node::<Particles>().start_emitting()
//! ```
//! Not quite as concise as GdScript, but still more concise than `gdnative-rust`. One thing to note:
//! this function almost literally translates to the code above. There is an explicit `unsafe` block,
//! and a variety of unwraps. This is very unsafe, but when will this fail? Only if you request an
//! invalid node, or break the memory model. Rust is designed to make you recover, but how do you
//! recover from a missing node at runtime? You will probably just `unwrap` anyways to appease the
//! compiler. This is called `unsafe_functions` because it is unsafe in the eyes of rust, but
//! when compared to GdScript, this is pretty normal and safe.
//!
//! You should definitely read about the panics each method can produce and understand
//! [`gdnative-rust`'s memory model](https://docs.rs/gdnative/0.9.3/gdnative/struct.Ref.html). Once
//! you do, you should have the right judgement on when to use these helper functions.
//!
//! # Compatibility
//! Unfortunately, `gdrust` requires the `gdnative` dependency, and it can not be `pub use`d due
//! to the way `gdnative`'s macros work. As as result, you must ensure you have a compatible version
//! of both `gdrust` and `gdnative`. This table will be updated with all compatible versions:
//!
//! | `gdrust`  | `gdnative-rust` |
//! |---------|----------|
//! | `0.1.0` | `0.9.+`  |
//!
//! # Additional Reading
//! - [Contributing](./CONTRIBUTING.md)
//! - [Reasoning for this project](./docs/why_gdrust.md)
//! - [FAQs](./docs/faq.md)
pub use gdrust_macros as macros;
pub mod unsafe_functions;
