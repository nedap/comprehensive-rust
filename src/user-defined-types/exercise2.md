---
minutes: 15
---

# Exercise: Divide function error handling

When dividing two numbers, not all values are possible. For example you can not divide by zero. Edit the following functions such that you neatly handle this case.

```rust,editable,should_panic

{{#include error_handling_exercise.rs:divide}}
    todo!()
}

{{#include error_handling_exercise.rs:DivideError}}
// TODO: Add possible errors when dividing two numbers
}

{{#include error_handling_exercise.rs:main}}
// TODO: Print if division succeeded, otherwise print error message
}
}
```
