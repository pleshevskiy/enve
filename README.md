# enve

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/pleshevskiy/enve/CI?label=tests&logo=github&style=flat-square)](https://github.com/pleshevskiy/enve/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/enve?style=flat-square)](https://docs.rs/enve)
[![Crates.io](https://img.shields.io/crates/v/enve?style=flat-square)](https://crates.io/crates/enve)
[![Crates.io](https://img.shields.io/crates/l/enve?style=flat-square)](https://github.com/pleshevskiy/enve/LICENSE)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square)](https://github.com/rust-secure-code/safety-dance/)
[![Matrix](https://img.shields.io/matrix/enve_team:matrix.org?label=matrix&style=flat-square)](https://matrix.to/#/!yZalHbWfGRWOMaetSn:matrix.org?via=matrix.org)

`enve` helps you work with environment variables and convert it to **any type**
using only **type annotations**.

Look at the [examples](https://github.com/pleshevskiy/enve/tree/main/examples)
to see the power!

All standard environment variable types are included, but `enve` under the hood
uses [estring](https://github.com/pleshevskiy/estring), so you can easily create
your own type.

## Getting started

```rust
use enve::SepVec;

type MinusVec<T> = SepVec<T, '-'>;
type PlusVec<T> = SepVec<T, '+'>;
type MulVec<T> = SepVec<T, '*'>;

fn main() -> Result<(), enve::Error> {
    enve::sset("E", "10+5*2-3");

    let res: f32 = enve::get::<PlusVec<MinusVec<MulVec<f32>>>>("E")
        .unwrap()
        .iter()
        .map(|p| {
            p.iter()
                .map(|m| m.iter().product::<f32>())
                .reduce(|acc, v| acc - v)
                .unwrap_or_default()
        })
        .sum::<f32>();

    println!("result: {}", res);

    Ok(())
}
```

## Installation

The MSRV is 1.51.0

Add `enve = { version = "0.1", features = ["prim", "vec"] }` as a dependency in
`Cargo.toml`.

`Cargo.toml` example:

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
enve = { version = "0.1", features = ["prim", "vec"] }
```

## License

**MIT**. See [LICENSE](https://github.com/pleshevskiy/estring/LICENSE) to see
the full text.

## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator,
maintainer.
