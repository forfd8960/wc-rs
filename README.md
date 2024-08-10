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

### Read from Stdin

```sh
~/D/C/g/f/wc-rs (main)> cat LICENSE |cargo run -- -l
   Compiling wc-rs v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.71s
     Running `target/debug/wc-rs -l`
Options { longest: false, lines: true, count: false, chars: false, words: false, files: [] }
stdin: 21 0 0 0
~/D/C/g/f/wc-rs (main)> cat LICENSE |wc -l
      21
~/D/C/g/f/wc-rs (main)> cat LICENSE |wc -w
     168
~/D/C/g/f/wc-rs (main)> cat LICENSE |cargo run -- -w
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/wc-rs -w`
Options { longest: false, lines: false, count: false, chars: false, words: true, files: [] }
stdin: 0 168 0 0
```
