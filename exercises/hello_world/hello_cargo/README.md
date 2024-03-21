Running ```cargo new hello_cargo``` from /exercises/hello_world/ created the following ```hello_cargo``` directory:

```bash
hello_world
├── hello_cargo
│   ├── Cargo.toml
│   └── src
│       └── main.rs
```

```bash
cargo build
./target/debug/hello_cargo # run the built binary
cargo run # builds and runs
cargo check # checks that code compiles but doesn't create a binary
cargo build --release # compile with optimizations
```
