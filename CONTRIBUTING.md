Contributions are always welcome! In general, there are a few ways to contribute:

1. Filing bugs
2. Making suggestions
3. Create PRs

### Filing Bugs
If you find a bug, please let me know. I make bugs a priority. When filing an issue,
please choose the appropriate issue template and include a sample as well as what you
think the expected output should be. If the issue involves the editor, a screenshot of
Godot would be great.

### Making Suggestions
This project will hopefully get better over time, and that starts with proposals. In 
general, please ensure proposals follow this project's philosophy of improving 90% of
use cases. We are trying to keep the syntax as close to GdScript as possible and are
favoring readability over supporting 100% of cases. Additionally, please try to mirror
Godot 4.0's syntax as much as possible.

### Creating PRs
PRs are always welcome. If you are making a PR to resolve a specific issue, please link 
to that issue. If you are adding a new feature, please consider filing an issue before 
you implement the change so it can be discussed by the community.

When making a PR, ensure the automated tests pass, but also test the example project:

1. Open `comprehensive_example` in Godot.
2. View the `HelloWorld` node in the inspector.
3. Look at each property, and ensure the hint seems to match the name.
4. Run the game. Ensure it says all tests pass.

### Creating new `unsafe_function`s
Additional `unsafe_function`s are always welcome. When designing a new one, please
consider the following:

1. **Does this replicate a gdscript behavior?** In general, we want to only add functions
that expose some gdscript behavior. For example, `require_typed_node()` functions the same
as `get_node()` in gdscript.
2. **How safe is it?** Yes, it is called `unsafe_functions`, but this is unsafe in the
context of rust. Much like the comment above, we should be trying to duplicate some 
gdscript feature as well as its risks. All panics should be clearly documented and 
intuitive given the function name, and all unsafe code should be documented and safe 
in standard use cases. Simply stating that a function calls `assume_safe()` is a good
enough hint for the user.