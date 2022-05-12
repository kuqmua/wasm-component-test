### add build target
```
rustup target add wasm32-unknown-unknown
```

### build
```
cargo build --target=wasm32-unknown-unknown --release
```
### run
```
python3 -m http.server
```
