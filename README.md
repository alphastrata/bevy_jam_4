## About:
....


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

### Developing;
- save the CI pipeline's juices by using the included `pre-commit`, dump a copy of it (with executable permissions) into your:
`./git/hooks` folder and it will run whenever you make commits, failing them if any of the jobs that would run in the CI pipe fail locally. (This is a much, much faster workflow than using GH's runners)


## TODO

* [ ] Map generation

## Attributions:
_ Stuff NOT made by the team, but from OS/Freely available resources_
- [electric.mp3, money.mp3] are provided by = www.Zapsplat.com
