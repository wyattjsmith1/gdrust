## FAQ
**Q**: I don't have experince in Rust; I just know GdScript. Can I just paste my scripts here for
fast code?

**A**: No. Not at all. This will never happen. There are a number of reasons why:

1. GdScript is dynamically typed, Rust is statically typed.
2. Rust and GdScript have completely different types. While there is some translation between
   them, we will likely never get 100% consistency.
3. Right now, this library only supports properties and signals. Logic is still handled in an
   `impl` block and is 100% rust.
4. Rust's memory model is 100% safe with gdscript. Check the [`assume_safe`](https://docs.rs/gdnative/latest/gdnative/struct.Ref.html#method.assume_safe)
   for more details.

It is recommended that all users be familiar with `gdnative-rust` before looking at `gdrust` as
`gdrust` is just some sugar to make `gdnative-rust` a bit cleaner. `gdrust` is not some magic
language that turns GdScript code into rust code to improve performance.

---

**Q**: Will this be pushed to [`crates.io`](crates.io)?

**A**: Eventually, yes. Right now, I am still in the early testing phase. If this turns out to
be useful to others and of beta quality, I will definitely push it to `crates.io`.

---

**Q**: Will this replace `gdnative-rust` in my project?

**A**: gdrust rides on top of gdnative, so you will need both side-by-side. Additionally, this
(currently) only supports properties and signals. Functions are still exported through an `impl`
block. Lastly, while this library does improve the exporting experience, it may not cover 100%
of cases. If that happens, you may need to use "plain" `gdnative-rust` for the "full feature
experience".

---

**Q**: Will this ever be merged with `gdnative-rust`?

**A**: While I am open to the idea, I don't think it's the right direction for `gdnative-rust`.
This macro defines code that is very unrust-like; that is the goal of this project.
`gdnative-rust` tends to focus more on the Rusty way to do things.

---

**Q**: Why create some abominable "language" halfway between Rust and GdScript? If users like
GdScript so much, they should just use it.

**A**: This is a good question. I don't see this library as being the best thing for every
project, but I do think it can greatly improve most projects. When I look at my games, I see that
the Godot Object makes up an insiginificant part of my logic. Much of my logic is delegated to
other "true rust" functions. This project is not intended to create a new language that complies
to rust, but rather to improve the bindings.
pt. I don't have strong feelings on this, so it may change if others do.
