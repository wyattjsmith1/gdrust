## FAQ
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
This project defines code that is either unsafe, or magic macros.
`gdnative-rust` tends to focus more on the Rusty way to do things.
