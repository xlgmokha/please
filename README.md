# please

A simple Rust library providing convenient builder pattern utilities.

## Overview

`please` offers two main functions for creating and initializing objects:

- `build()` - Creates a new instance using the type's `Default` implementation
- `build_with()` - Creates a new instance and applies a custom initializer function

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
please = "0.1.0"
```

### Examples

Basic usage with `build()`:

```rust
use please::build;

#[derive(Default)]
struct Person {
    name: String,
    age: i32,
}

let person = build::<Person>();
// Creates Person { name: "", age: 0 }
```

Custom initialization with `build_with()`:

```rust
use please::build_with;

let person = build_with(|p: &mut Person| {
    p.name = String::from("Alice");
    p.age = 30;
});
// Creates Person { name: "Alice", age: 30 }
```

## Requirements

- Rust 2024 edition
- Types must implement `Default` trait

## License

MIT