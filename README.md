# offset_of

## Usage

```
[dependencies]
offset_of = { git = "https://github.com/woppopo/offset_of" }
```

```rust
use offset_of::offset_of;

struct Structure {
	val1: u32,
	val2: u32,
}

const OFFSET_OF_VAL2: usize = offset_of!(Structure, val2);
```