# `size-display`

Display human readable file sizes.

## Limitation

Displayed units go up to Exabyte (2^60).

## Example

```rust
use size_display::Size;

assert_eq!("24", format!("{}", Size(24)));
assert_eq!("4.2G", format!("{}", Size(4509715660)));
```
