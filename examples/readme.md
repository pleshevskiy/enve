# Diesel

This example shows how you can use itconfig with diesel.

### Usage

```bash
cd examples/diesel

docker-compose -p itconfig-diesel-example -f docker-compose.example.yml up -d

diesel migration run

cargo run --example diesel
```

# Hyper

This example shows how you can use itconfig with hyper server.

Example was taken from the [official hyper example].

[official hyper example]: https://github.com/hyperium/hyper/blob/master/examples/web_api.rs

### Usage

```bash
cargo run --example hyper
```

# Rocket

### Installation

To install a nightly version of Rust, we recommend using rustup. Install rustup
by following the instructions on its website. Once rustup is installed,
configure Rust nightly as your default toolchain by running the command:

`rustup default nightly`

If you prefer, once we setup a project directory in the following section, you
can use per-directory overrides to use the nightly version only for your Rocket
project by running the following command in the directory:

`rustup override set nightly`

### Usage

```bash
cargo run --example roket
```
