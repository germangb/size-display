# size-display

[![Crates.io](https://img.shields.io/crates/v/size-display?style=flat-square)](https://crates.io/crates/size-display)

Display human readable file sizes.

## Example

```rust
use size_display::Size;

assert_eq!("24", format!("{}", Size(24)));
assert_eq!("4.2G", format!("{:.1}", Size(4509715660)));
```
