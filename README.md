# read_iter

[![Documentation](https://docs.rs/read_iter/badge.svg)](https://docs.rs/read_iter)
[![crates.io](https://img.shields.io/crates/v/read_iter.svg)](https://crates.io/crates/read_iter)

To any `std::io::Read` implementor, add also `Iterator<Item=u8>` implementation.

## Installation

In `Cargo.toml` of your project add:
```toml
[dependencies]
read_iter = "0.1"
```

## Examples

```rust
use std::fs::File;
use read_iter::ReadIter;

let file = File::open("/tmp/test.txt").unwrap();
// "file" implements std::io::Read
let mut it = ReadIter::new(file);
// now "it" also implements std::io::Read
// and "&mut it" implements Iterator<Item=u8>
// also "it" has internal buffer, and implements std::io::BufRead
for byte in &mut it
{	// ...
}
it.take_last_error().unwrap();
```
