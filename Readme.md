# Build
## Web
```
wasm-pack build --target web
cd pkg
python3 -m http.server
```

## Desktop
```
cargo build
cd target/debug
./hello_golem
```
Note: the last step depends slightly on the operating system.