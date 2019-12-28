# Diesel

This example shows how you can use itconfig with diesel.


### Usage

```bash
cd examples/diesel

docker-compose -p itconfig-diesel-example -f docker-compose.example.yml up -d

diesel migration run

cargo run
```