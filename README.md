# Yangon

A high-performance, stack-allocated string type for Rust with fixed capacity and zero heap allocations.

[![Crates.io](https://img.shields.io/crates/v/yangon.svg)](https://crates.io/crates/yangon)
[![Docs.rs](https://docs.rs/yangon/badge.svg)](https://docs.rs/yangon)
[![License](https://img.shields.io/crates/l/yangon?style=flat)](https://github.com/rustersai/yangon/blob/main/LICENSE)



## Overview

Yangon is a production-ready string library that provides a stack-allocated alternative to `String`. By using fixed-capacity storage and const generics, Yangon eliminates heap allocations for performance-critical applications while maintaining a familiar String-like API.

**Performance Highlights:**
-  **25x faster** than `std::string::String` for `push_str` operations
-  Zero heap allocations - all data stored on the stack
-  Configurable capacity via const generics
-  Full UTF-8 validation and support

## Installation

Add Yangon to your `Cargo.toml`:

```toml
[dependencies]
yangon = "0.0.8"
```

## Quick Start

```rust
use yangon::{Yangon, yangon};

// Create with default 10KB capacity
let mut s = Yangon::new();
s.push_str("Hello, ").unwrap();
s.push_str("Yangon!").unwrap();
println!("{}", s); // "Hello, Yangon!"

// Or use the macro
let s = yangon!("Hello, World!");

// Custom capacity with const generics
let mut small: Yangon<64> = Yangon::with_capacity();
small.push_str("Small buffer").unwrap();
```

## Key Features

### Stack Allocation
- **No heap allocations** - all string data lives on the stack
- **Predictable memory usage** - capacity known at compile time
- **Default 10KB capacity** when using `Yangon::new()` or `Yangon::from()`
- **Configurable via const generics** - `Yangon<N>` where N is byte capacity

### String-Like API

Yangon provides familiar methods similar to `String`:

```rust
let mut s = yangon!("Hello");

// Push operations
s.push_str(" World").unwrap();
s.push('!').unwrap();

// Modification
s.insert(5, ',');
s.remove(5);
s.pop();
s.clear();
s.truncate(5);

// Inspection
assert_eq!(s.len(), 5);
assert!(!s.is_empty());
assert_eq!(s.as_str(), "Hello");
```

### Advanced Pattern Matching

Yangon's `replace` function supports multiple pattern types with turbo fish syntax:

```rust
let s = yangon!("Hello World");

// Replace string slice
let s1 = s.replace::<&str, 0>("World", "Yangon");

// Replace single character
let s2 = s.replace::<char, 0>('o', "0");

// Replace multiple characters
let s3 = s.replace::<_, 2>(&['H', 'W'], "X");

// Replace with closure (e.g., remove whitespace)
let s4 = s.replace::<fn(char) -> bool, 0>(|c| c.is_whitespace(), "");
```

### UTF-8 Handling

Full UTF-8 support with validation:

```rust
use yangon::Yangon;

// From valid UTF-8
let s = Yangon::from_utf8(vec![72, 101, 108, 108, 111]).unwrap();

// Unchecked (unsafe but faster)
let s = unsafe { Yangon::from_utf8_unchecked(vec![72, 101, 108, 108, 111]) };

// Lossy conversion (replaces invalid sequences with �)
let bytes = vec![0xFF, 0xFE, 72, 105];
let s = Yangon::from_utf8_lossy(&bytes);
```

### Iteration and Collection

```rust
use yangon::Yangon;

// From iterator
let chars = vec!['H', 'e', 'l', 'l', 'o'];
let s: Yangon = chars.into_iter().collect();

// Retain with predicate
let mut s = yangon!("Hello123World");
s.retain(|c| c.is_alphabetic());
assert_eq!(s, "HelloWorld");
```

### Additional Operations

```rust
let mut s = yangon!("  Hello World  ");

// Trimming (returns &str)
assert_eq!(s.trim(), "Hello World");

// Split off at index
let s2 = s.split_off(7);

// Replace range
s.replace_range(0..5, "Hi");

// Convert to bytes
let bytes: Vec<u8> = s.into_bytes();

// Convert to String
let string: String = s.to_string();
```

## Use Cases

Yangon is ideal for:

- **Performance-critical code** where heap allocation overhead is unacceptable
- **Network protocols** requiring fixed-size string buffers
- **Data transfer/storage** with known maximum string lengths
- **Real-time applications** requiring predictable performance

### Real-World Example

Currently used in production for a Facebook Reel downloader application, handling URL manipulation and data processing with consistent performance.

## Performance Considerations

### When Yangon Excels
-  `push_str` operations: **~25x faster** than `String`
-  `push` operations: Significantly faster
-  Character-by-character building: Minimal allocation overhead
-  Short to medium strings fitting in capacity

### Performance Notes
-  `replace` function is slower than `String::replace` - avoid in hot paths when possible
-  Fixed capacity means capacity overflow returns `Err(yError::CapacityOverflow)`
-  Best for data transfer and storage; consider `String` for highly dynamic string manipulation

## Safety

Yangon uses `unsafe` internally for performance-critical operations but maintains safety guarantees:

-  **UTF-8 validation** on all public APIs (except `_unchecked` variants)
-  **Bounds checking** prevents invalid memory access
-  **Production tested** under stress conditions
-  Users must respect capacity limits - overflows return errors or panic
-  `_unchecked` variants assume valid input for maximum performance

**Responsibility:** Yangon is not as flexible as `String` due to fixed capacity. Use with understanding of your data size requirements.

## API Conversion Reference

| String Method | Yangon Equivalent | Returns |
|---------------|-------------------|----------|
| `String::new()` | `Yangon::new()` | `Yangon` |
| `String::from(s)` | `Yangon::from(s)` | `Yangon` |
| `s.push_str(s)` | `s.push_str(s)` | `Result<(), yError>` |
| `s.push(c)` | `s.push(c)` | `Result<(), yError>` |
| `s.as_str()` | `s.as_str()` | `&str` |
| `String::from_utf8(v)` | `Yangon::from_utf8(v)` | `Result<Yangon, yError>` |
| `String::from_utf8_lossy(v)` | `Yangon::from_utf8_lossy(v)` | `yCow<Yangon>` |

## Traits Implemented

- `Display` - Format for printing
- `Debug` - Debug formatting
- `Write` - Format writing support
- `Deref<Target = str>` - Automatic coercion to `&str`
- `AsRef<str>` - Borrow as string slice
- `PartialEq<&str>` - Compare with string slices
- `FromIterator<char>` - Build from character iterator
- `Clone` - Deep copy support

## Error Handling

```rust
use yangon::yError;

match s.push_str("test") {
    Ok(()) => println!("Success"),
    Err(yError::CapacityOverflow) => println!("Buffer full!"),
    Err(yError::FromUtf8Error) => println!("Invalid UTF-8"),
}
```

## Macro Usage

The `yangon!` macro provides convenient initialization:

```rust
// Empty Yangon
let empty = yangon!();

// With initial content
let s = yangon!("Hello, World!");

// Note: Only accepts 0 or 1 string literal
```

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/yangon).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

This is a stable, production-ready project. While it's not actively seeking contributors, bug reports and suggestions are welcome via GitHub issues.

## About the Name

Yangon is named after the capital city of Myanmar. The name reflects the project's foundation: solid, reliable, and built for real-world use.

---

**Note:** Yangon prioritizes performance over flexibility. Understand your string size requirements before choosing Yangon over `String`. When used appropriately, it provides significant performance benefits with zero heap allocation overhead.
