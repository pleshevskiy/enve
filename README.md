# enve

[![Crates.io](https://img.shields.io/crates/v/enve?style=flat-square)](https://crates.io/crates/enve)
[![docs.rs](https://img.shields.io/docsrs/enve?style=flat-square)](https://docs.rs/enve)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/pleshevskiy/enve/CI?label=tests&logo=github&style=flat-square)](https://github.com/pleshevskiy/enve/actions/workflows/ci.yml)
![The MSRV](https://img.shields.io/badge/MSRV-1.59.0-red.svg)

```toml
[dependencies]
enve = "0.3"
```

`enve` helps you work with environment variables and convert it to **any type**
using only **type annotations**.

All standard environment variable types are included, but `enve` under the hood
uses [estring](https://github.com/pleshevskiy/estring), so you can easily create
your own type.

## [Documentation](https://docs.rs/enve)

Look at the [examples] to see the power!

[examples]: https://github.com/pleshevskiy/enve/tree/main/examples

## Usage

Basic

```rust
fn main() -> Result<(), enve::Error> {
    enve::sset("E", "10");

    let res: f32 = enve::get("E")?;

    println!("result: {}", res);

    Ok(())
}
```

You can use predefined structs like `SepVec` if you enable `structs` feature.

Note: You can use custom types as annotations! Just implement `ParseFragment`.

```rust
use enve::SepVec;

type PlusVec<T> = SepVec<T, '+'>;
type MulVec<T> = SepVec<T, '*'>;

fn main() -> Result<(), enve::Error> {
    enve::sset("E", "10+5*2+3");

    let res: f32 = enve::get::<PlusVec<MulVec<f32>>>("E")
        .unwrap()
        .iter()
        .map(|m| m.iter().product::<f32>())
        .sum::<f32>();

    assert_eq!(res, 23.0);

    Ok(())
}
```

You can also use predefined aggregators if you enable `aggs` feature.

```rust
use enve::{SepVec, Product, Sum, estring::Aggregate};

type PlusVec<T> = SepVec<T, '+'>;
type MulVec<T> = SepVec<T, '*'>;

fn main() -> Result<(), enve::Error> {
    enve::sset("E", "10+5*2+3");

    let res: f32 = enve::get::<Sum<PlusVec<Product<MulVec<f32>>>>>("E")?
        .agg();

    assert_eq!(res, 23.0);

    Ok(())
}
```

## Contact Us

Join us in:

[![Matrix](https://img.shields.io/badge/matrix-%23enve_team:matrix.org-blueviolet.svg?style=flat-square)](https://matrix.to/#/#enve_team:matrix.org)

## License

**MIT**. See [LICENSE](https://github.com/pleshevskiy/estring/LICENSE) to see
the full text.

## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator,
maintainer.
