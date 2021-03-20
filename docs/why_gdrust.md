# Reasoning For This Project
The team at [`gdnative-rust`](https://github.com/godot-rust/godot-rust) has done a wonderful job
of making it possible to add Rust code to Godot. The main goal of their project is to enable
developers to create safe, Rusty code for Godot, and that is now a reality.

One of the shortcommings of `gdnative-rust` is that it is, well, very Rusty. Rust is a great
language for many things, but the translation with Godot becomes a bit rocky sometimes. One such
example is Rust's separation of data and logic through `struct`s and `impl`s. This undoubtedly
leads to better code, but doesn't work too well with Godot, which uses `class`es because
properties are part of the data (`struct`), but exposed in logic (`impls`).

To surface an example: exporting properties. Currently, using `gdnative-rust` we must export
properties like:
```rust
#[derive(gdnative::NativeClass)]
#[inherit(MeshInstance)]
#[register_with(register_properties)]
struct RustTest {
    test: u64,
}
fn register_properties(builder: &ClassBuilder<RustTest>) {
    builder
        .add_property::<String>("test/test_enum")
        .with_hint(IntHint::Enum(EnumHint::new(vec![
            "Hello".into(),
            "World".into(),
            "Testing".into(),
        ])))
        .with_getter(|this: &RustTest, _| this.test)
        .with_setter(|this: &mut RustTest, _, value| this.test = value)
        .done();
}
```
Users may optionally use the `#[property]` macro, but that prevents the user from creating
signals, and exposes very little in the way of export hints.

When you compare the block above to the equivalent GdScript, you see there is a huge difference:
```gdscript
class_name RustTest
extends MeshInstance
@export_enum("Hello", "World", "Testing") var test
```

To offer another example, let's look at signals:
```rust
#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
struct SignalEmitter {
    timer: f64,
    data: i64,
}
#[methods]
impl SignalEmitter {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "tick_with_data",
            args: &[SignalArgument {
                name: "data",
                default: Variant::from_i64(100),
                export_info: ExportInfo::new(VariantType::I64),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }
}
```
And the equivalent GdScript:
```gdscript
class_name SignalEmitter
extends Node
signal tick_with_data(data)
```
Again, GdScript is substantially more concise. At the bottom of the `gdrust` documentation is a
sample script written using gdrust that is about 50 lines. When expanded and formatted, it
expands to over 750 lines! That's almost 15x larger!!!

So, why did `gdnative-rust` create this ridiculously verbose way of exporting properties? Well,
they are just mirroring GdNative's property interface and keeping it Rusty. There is nothing
wrong with this method (in fact, it gives developers full functionallity), but it can be
overwhelming when compared to GdScript or most other Godot-supported languages.

Some developers may be torn between the saftey of Rust and the speed and conciseness of
GdScript. This library attempts to take the best of both Rust and GdScript to enable concise,
fast, and safe code for Godot.
