## parameters.rs

```sh
echo -en "#[rustfmt::skip]\npub mod parameters {\n    #[allow(dead_code)]\n    pub const PATTERN_INSTANCES: [f32; 167_265] = [" > parameters.rs
cat ../../python/data/parameters/0925.txt >> parameters.rs
echo -e "];\n}" >> parameters.rs
```

## Debugging
panic の詳細を DevTools に出したい場合は feature を有効化します。

- feature: `debug-panic-hook`
- hook 初期化: `newBoard` / `Game::create` 呼び出し時に `set_once()` を実行

```sh
wasm-pack build --dev -- --features debug-panic-hook
```
