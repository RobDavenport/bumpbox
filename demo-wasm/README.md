# demo-wasm

Browser showcase for `bumpbox` with deterministic 2D and 3D query scenes.

## Build

```bash
wasm-pack build demo-wasm --target web --release --out-dir www/pkg
```

## Run

```bash
cd demo-wasm/www
python -m http.server 8080
```

Open `http://localhost:8080`.
