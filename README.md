# rs2048
A clone of 2048 written in Rust. Use the arrow keys to move and R to reset.

This is work in progress but if you want to jump right in, take a look at this [demo](https://kappadistributive.github.io/rs2048/).

# How to run
Install `cargo-web` using `cargo` as

``` shell
cargo install -f cargo-web
```

Then run the application by

``` shell
cargo web start --target=wasm32-unknown-unknown
```

Then go to `http://127.0.0.1:8000` to view the application.
