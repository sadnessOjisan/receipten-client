# receipten-client

receipten のフロント

```sh
# dev
miniserve ./public --index index.html

# build
wasm-pack build --target web --out-name wasm --out-dir ./public
```

## TODO

- [x] routing から id 取得
- [x] id を使ってデータ取得
- [x] データを UI に表示
