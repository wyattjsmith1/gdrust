# gdrust

[![Rust](https://github.com/wyattjsmith1/gdrust/actions/workflows/rust.yml/badge.svg?branch=master&event=push)](https://github.com/wyattjsmith1/gdrust/actions/workflows/rust.yml)

A library for making [`gdnative-rust`](https://github.com/godot-rust/godot-rust) a bit more
GdScript-like. This contains two main parts:

1. A `#[gdrust]` macro for simplifying some rust code and making it more GdScript-like.
2. A set of "unsafe" functions to make things more concise at the risk of crashing.

## Goals
Ultimately, the goal of this project is rust development for Godot more concise in 90% of cases. There may
be some edge cases only "true" rust can resolve, and this project should not comprimise its
simplicity for the sake of covering every case.

## Current State
Right now, this project is in an early alpha state. The documented parts should work as expected,
but the api is likely to change.

## Getting Started
`gdrust` surfs on [`gdnative-rust`](https://github.com/godot-rust/godot-rust), so you must have
[`gdnative-rust`](https://github.com/godot-rust/godot-rust) setup before you start looking at
`gdrust`. Follow their [Getting Started Guide](https://godot-rust.github.io/#installation).

Once `gdnative-rust` is installed, you can install `gdrust` by adding it as a dependency.
Unfortunately, due to the way `gdnative-rust` macros work, you must have both `gdnative-rust`
and `gdrust` added as dependencies side-by-side, and you must choose compatible versions. See the
"Compatibilty" section below.
```toml
[dependencies]
gdnative = "0.9.3"
gdrust = { git = "https://github.com/wyattjsmith1/gdrust.git" }
```

Once installed, simply use the `gdrust` macro:
```rust
use gdrust::macros::gdrust;
use gdnative::api::Node;

#[gdrust(extends = Node)]
struct HelloWorld {
    #[export]
    #[default(10)]
    test: u64,
}
```
That's it!

Read more below for details and gotchas with exporting properties and signals, as well as an
in-depth comprehensive example.

## `#[gdrust]` Macro

### Exporting classes
Anything in a `#[gdrust]` macro is avaliable for export.
```rust
#[gdrust(extends = Node)]
pub struct ClassName {
   // Same as `class_name ClassName extends Node` in GdScript.
}
```
The `extends = {classname}` is optional, and may be omitted if you are just extending `Object`:
```rust
#[gdrust::macros::gdrust]
struct ClassName {
    // Same as `class_name ClassName extends Object` in GdScript.
}
```

You can still have custom derives and attributes on your class. Any attributes on the `struct` will
be added:
```rust
#[gdrust::macros::gdrust]
#[derive(Debug)]
struct ClassName {
    // `ClassName` will derive `Debug`
}
```

After you create the class and export properties and signals, create your `impl` block as
usual. Note, you should not create the `new` function. That is provided by the macro:
```rust

#[gdnative::methods]
impl ClassName {
    #[export]
    fn _ready(&self, _owner: TRef<KinematicBody>) {
        gdnative::godot_print!("Hello World!")
    }
}
```

### Exporting Properties
The syntax for exporting properties is intended to mirror GdScript as closely as possible. Due
to the upcoming 4.0 release, `gdrust` uses the [4.0 exports](https://docs.godotengine.org/en/latest/tutorials/scripting/gdscript/gdscript_exports.html).
You can read all about the different types of exports there.

In general, use attribute syntax (`#[export_...]`), and remove the `@` at the start of GdScript
export. For example:
```gdscript
@export_range(1, 10, 2, "or_greater") var my_range: int
```
Becomes:
```rust
#[export_range(1, 10, 2, "or_greater")]
my_range: i32 // or i64 if you want
```

Everything should be implemented as defined in Godot's docs except for the following:

1. `#[no_export]` can be used to not export a variable. This should be used for all Rust-native
types (doesn't implement `Export`) or if you want the variable to be "private".
2. The 4.0 docs define `@export_node_path(Type1, Type2)` as a way to export a `NodePath` which
only matches nodes with given types. This is partially implemented, but won't be finished until
4.0 because there is currently not export hint for NodePaths. You can currently include this
export in your code, but it will allow a `NodePath` to any type.
3. Nullability is handled with an `Option`.
4. Every exported property will require both a type and a default value. If no default value is
provided, `Default::default()` will be used. If you are referencing a Godot
object and not a "primitive", this must be wrapped in a `Ref`.
5. Currently, arrays are not supported. This is simply because I am not confident the syntax
has been finalized. On Godot's site, it shows the traditional `export(Array, int) var ints = [1, 2, 3]`.
I am guessing they will switch to some sort of `@export_array` style. Once that is finalized,
adding it should be easy.

#### Default
You may set a custom default value using the `#[default(value)]` annotation. If it is not defined,
`Default::default()` is used.

### Exporting Signals
The syntax for exporting signals is also intended to mirror [GdScript](https://docs.godotengine.org/en/latest/getting_started/step_by_step/signals.html#custom-signals)
as closely as possible. The syntax is:
```rust
#[gdrust]
#[signal(signal_name(arg_name: I64, arg2_name: F64 = 10.0))]
#[signal(other_signal(arg_name: Bool = true, arg2_name: GodotString = "default"))]
struct Class;
```

Similar to properties, there are a few gotchas with signals:

1. Like properties, every signal must have a type. Unlike properties, the type must be one of:
  - A [`VariantType`](https://docs.rs/gdnative/0.9.3/gdnative/core_types/enum.VariantType.html)
  - A Godot object without a `Ref` (like a `KinematicBody`).

  I know this is a little weird, and I'd like to smooth it out a bit. Suggestions are welcome.

2. Unlike GdScript, `gdrust` signal arguments may have optional default values.

When a signal is exported, there will be a `const` with its name. Look at the `simple_signal`
signal in the example below to see how it can be used.

### Comprehensive Example
This example should contain all possibilities for exporting properties and signals. It is used
for testing as well.
```rust
use gdnative::api::{KinematicBody, Node, RigidBody, Texture};
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

    #[export_range(0.0, 10.0)]
    simple_range: f32,

    #[export_range(0, 10, 2)]
    #[default(2)]
    step_range: u8,

    #[export_range(0, 10, "or_lesser")]
    #[default(10)]
    simple_range_or_lesser: u64,

    #[export_range(0.0, 10.0, 1.5, "or_lesser")]
    #[default(10.0)]
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
    fn _ready(&self, owner: TRef<Node>) {
        gdnative::godot_print!("Hello World!");
        gdnative::godot_dbg!(self);
        owner
           .upcast::<Node>()
           .emit_signal(Self::SIMPLE_SIGNAL, &[0.to_variant()]);
    }
}
```

### Pros and Cons
Like any piece of software, this is not without it's issues. This list is intended to accurately
document the pros and cons to help people decide if this is the right project for them.

#### Pros

1. Simplifies the `ClassBuilder` chain and makes the code look more GdScripty
2. Generates a `new`
3. Synchronizes the property default with the `new` default. No more changing the default
property value and not having it reflected in code.

#### Cons
1. Like many macros, when the input is correct, they work great. When the input is invalid,
they give obscure error messages. I am trying to cover most of the common error cases with clear
messages. If you see weird message, open an issue and I will help you out. In general, `#[export*`s
should always use the same type of literals (all ints or all floats).

## Unsafe Functions
One of the great things about rust is that it forces you to handle every possible case to ensure
the runtime goes smoothly. One issue with this is game development is full of "well, I hope this
works" cases in which error handling is ignored until runtime.

For example, let's say you want to get a child node and call `set_emitting()` on it. In
`gdnative-rust`, you would do this:
```rust
unsafe {
    owner.get_node("Particles")
        .unwrap()
        .assume_safe()
        .cast::<Particles>()
        .unwrap()
        .set_emitting(true);
}
```
Compare to GdStript (without the $ sugar):
```gdscript
get_node("Particles").start_emitting()
```
Yes, the static typing does cause some verbosity in the rust example, but this is still a lot.
`gdrust` exposes a cleaner method:
```rust
owner.expect_node::<Particles, _>("Particles").set_emitting(true)
```
Not quite as concise as GdScript, but still more concise than `gdnative-rust`. One thing to note:
this function almost literally translates to the code above. There is an explicit `unsafe` block,
and a variety of unwraps. This is very unsafe, but when will this fail? Only if you request an
invalid node, or break the memory model. Rust is designed to make you recover, but how do you
recover from a missing node at runtime? You will probably just `unwrap` anyways to appease the
compiler.

As a result, this is called `unsafe_functions` because it is unsafe in the eyes of rust, but
when compared to GdScript, this is pretty normal and safe.

You should definitely read about the panics each method can produce and understand
[`gdnative-rust`'s memory model](https://docs.rs/gdnative/0.9.3/gdnative/struct.Ref.html). Once
you do, you should have the right judgement on when to use these helper functions.

## Compatibility
Unfortunately, `gdrust` requires the `gdnative` dependency, and it can not be `pub use`d due
to the way `gdnative`'s macros work. As as result, you must ensure you have a compatible version
of both `gdrust` and `gdnative`. This table will be updated with all compatible versions:

| `gdrust`  | `gdnative-rust` |
|---------|----------|
| `0.1.0` | `0.9.+`  |

## Additional Reading
- [Contributing](./CONTRIBUTING.md)
- [Reasoning for this project](./docs/why_gdrust.md)
- [FAQs](./docs/faq.md)

License: MIT
