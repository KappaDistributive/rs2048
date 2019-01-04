# rs2048
A clone of 2048 written in Rust. Use the arrow keys to move and R to reset.

This is work in progress but if you want to jump right in, take a look at this [demo](https://kappadistributive.github.io/rs2048/).

#  Running the application
1. Install `cargo-web` using `cargo` as

``` shell
cargo install -f cargo-web
```

2. Compile to [WebAssembly](https://en.wikipedia.org/wiki/WebAssembly) using Rust's native WebAssembly backend:

``` shell
cargo web start --target=wasm32-unknown-unknown
```


3. Visit `http://localhost:8000` with your browser to view the application.


For more information, see the [Github repository](https://github.com/koute/stdweb/blob/master/README.md) and [documentation](https://docs.rs/stdweb/*/stdweb/) of stdweb.
