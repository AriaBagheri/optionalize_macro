
# Optionalize Macro

`optionalize_macro` is a procedural macro crate for Rust that generates a new struct with all fields wrapped in `Option`. If a field is already an `Option<T>`, it remains as `Option<T>`; otherwise, it’s wrapped as `Option<T>`. This is useful for creating structs for partial updates, where only some fields may need to be modified.

## Features

- Automatically generates an "optionalized" version of a struct.
- Retains `Option` types if they are already present in the original struct.
- Can be useful for partial updates or optional struct fields.

## Installation

Add `optionalize_macro` to your `Cargo.toml`:

```toml
[dependencies]
optionalize_macro = { path = "../optionalize_macro" }
```

> **Note:** Ensure this path points to where `optionalize_macro` is located in your project. You can adjust it based on your directory structure.

## Usage

To use the `Optionalize` macro, simply derive it on your struct. A new struct will be generated with the same name, appended with `Optional`, where all fields are wrapped in `Option`.

### Example

Here’s a basic example demonstrating how to use `Optionalize`.

```rust
use optionalize_macro::Optionalize;

#[derive(Optionalize)]
pub struct MyStruct {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

// The generated struct will be:
// pub struct MyStructOptional {
//     pub id: Option<i32>,
//     pub name: Option<String>,
//     pub description: Option<String>,
// }
```

### Example Usage in Code

With `Optionalize`, you can create an "optionalized" struct for scenarios where only certain fields are updated:

```rust
use optionalize_macro::Optionalize;

#[derive(Optionalize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
}

fn main() {
    // Original struct
    let user = User {
        id: 1,
        username: "user123".to_string(),
        email: Some("user@example.com".to_string()),
    };

    // Partial update with optional fields
    let user_update = UserOptional {
        id: None,  // We don't want to update `id`
        username: Some("new_user123".to_string()),  // Update `username`
        email: None,  // No change to `email`
    };

    // Now, `user_update` can be used for partial updates where only certain fields are modified
}
```

## How It Works

The `Optionalize` macro inspects each field in your struct:

- If the field is already an `Option<T>`, it keeps it as `Option<T>`.
- If the field is of type `T`, it wraps it as `Option<T>`.

This allows for flexible use cases where you only want to update a subset of fields in your struct without needing to specify every field explicitly.

## Limitations

- The `Optionalize` macro only works with structs and does not support enums.
- It requires the `syn` and `quote` crates for parsing and generating Rust code.

## Development and Testing

Since procedural macros cannot be tested within the same crate that defines them, tests for `Optionalize` must be written as integration tests.

1. Create a `tests` directory if it doesn’t exist.
2. Add a test file (e.g., `optionalize_macro_test.rs`) with examples and assertions.

Here is a sample test file:

```rust
// tests/optionalize_macro_test.rs
use optionalize_macro::Optionalize;

#[derive(Optionalize, Debug, PartialEq)]
struct TestStruct {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[test]
fn test_optionalize_macro() {
    #[derive(Debug, PartialEq)]
    struct TestStructOptional {
        pub id: Option<i32>,
        pub name: Option<String>,
        pub description: Option<String>,
    }

    let original = TestStruct {
        id: 1,
        name: "example".to_string(),
        description: Some("description".to_string()),
    };

    let expected = TestStructOptional {
        id: Some(1),
        name: Some("example".to_string()),
        description: Some("description".to_string()),
    };

    let optionalized = TestStructOptional {
        id: Some(original.id),
        name: Some(original.name.clone()),
        description: original.description.clone(),
    };

    assert_eq!(optionalized, expected);
}
```

Run tests with:

```bash
cargo test
```

## License

This project is licensed under the MIT License.
