# wc-rs


## Example

```sh
cargo run -- -l -w -f Cargo.toml Cargo.lock

Options { longest: false, lines: true, count: false, chars: false, words: true, files: ["Cargo.toml", "Cargo.lock"] }
Cargo.toml: 8
Cargo.lock: 244
Cargo.toml: 20
Cargo.lock: 492
Cargo.lock: 244 492 0 0
Cargo.toml: 8 20 0 0
```
