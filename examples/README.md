# Examples

## Calculator

Fun calculator based on environment variables

```
E=2*2-1-1+5*3-10 cargo run --example calc --all-features
```

Limits:

- Supports `*`, `+`, `-`
- You cannot start from a negative number. `E=-10`. Solution: start from `0`.
  `E=0-10`.
