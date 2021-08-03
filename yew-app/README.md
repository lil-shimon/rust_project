## wasm app

### dev

```
wasm-pack build --target web --out-name wasm --out-dir ./static
cargo +nightly install miniserve
miniserve ./static --index index.html
```

then go to [http://127.0.0.1:8080/](http://127.0.0.1:8080/)
