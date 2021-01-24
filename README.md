# receipten-client

receipten のフロント

```sh
# dev
miniserve ./public --index index.html

# build
wasm-pack build --target web --out-name wasm --out-dir ./public
```

## TODO

- [ ] routing から id 取得
- [ ] id を使ってデータ取得
- [ ] データを UI に表示
