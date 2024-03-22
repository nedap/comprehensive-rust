---
minutes: 8
---

# Destructuring

Like tuples, structs and enums can also be destructured by matching:

## Enums

Patterns can also be used to bind variables to parts of your values. This is how
you inspect the structure of your types. Let us start with a simple `enum` type:

```rust,editable
use rand::Rng;

enum Action {
    Walk(u32),
    Turn(f64),
}

fn random_action() -> Action {
    let mut rng = rand::thread_rng();

    match rng.gen_range(1..3) { // Random value between 1 and 2
      1 => Action::Walk(rng.gen_range(1..100)), // Generate a random u32 between 1 and 99
      2 => Action::Turn(rng.gen_range(0.0..360.0)), // Generate a random f64 between 0.0 and 359.9
      _ => panic!("oh no"),
    }
}

fn main() {
    let action = random_action();
    match action {
      Action::Walk(distance) => println!("Walking {distance} miles"),
      Action::Turn(degrees) => println!("Turning {degrees:.2} degrees"),
    }
}
```

Here we have used the arms to _destructure_ the `Action` value. In the first
arm, `distance` is bound to the value inside the `Walk` variant. In the second arm,
`degrees` is bound to the `Turn` variant.


## Structs

```rust,editable
{{#include ../../third_party/rust-by-example/destructuring-structs.rs}}
```

<details>

# Enums

Key points:

- The `if`/`else` expression is returning an enum that is later unpacked with a
  `match`.
- You can try adding a third variant to the enum definition and displaying the
  errors when running the code. Point out the places where your code is now
  inexhaustive and how the compiler tries to give you hints.
- The values in the enum variants can only be accessed after being pattern
  matched.
- Demonstrate what happens when the search is inexhaustive. Note the advantage
  the Rust compiler provides by confirming when all cases are handled.
- Save the result of `divide_in_two` in the `result` variable and `match` it in
  a loop. That won't compile because `msg` is consumed when matched. To fix it,
  match `&result` instead of `result`. That will make `msg` a reference so it
  won't be consumed. This
  ["match ergonomics"](https://rust-lang.github.io/rfcs/2005-match-ergonomics.html)
  appeared in Rust 2018. If you want to support older Rust, replace `msg` with
  `ref msg` in the pattern.

# Structs

- Change the literal values in `foo` to match with the other patterns.
- Add a new field to `Foo` and make changes to the pattern as needed.
- The distinction between a capture and a constant expression can be hard to
  spot. Try changing the `2` in the second arm to a variable, and see that it
  subtly doesn't work. Change it to a `const` and see it working again.

</details>
