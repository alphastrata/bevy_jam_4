## Usage

### Step 1.
>_ASSUMING YOU HAVE RUST AND CARGO ETC CORRECTLY INSTALLED_
```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-server-runner
```

### Step 2.

Run programs in the browser using
```sh
cargo run --target wasm32-unknown-unknown
# or if compiled already:
wasm-server-runner path/to/file.wasm
```