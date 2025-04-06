# suicide-rs
A super simple crate for printing an error message when something terrible goes wrong and nothing can be done to recover.

## Example
```rust
fn main() {
    let val1: u8 = 10;
    let val2: u8 = 20;
    if (val1 + val2) != 35 {
        suicide_rs::die!(EINVAL, "It is good day to be not dead!");
    }
    unreachable!("You are dead!");
}
```

This crate represents about half an hour of work, so this is really only relevant for when you're really lazy.
