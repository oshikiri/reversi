# Repository Guidelines

- `engine/` 以下を変更した場合は、 `cargo test` `cargo clippy -- -D warnings` `wasm-pack build` を実行して問題ないことを確認する。
- Codexでの実行時
  - binaryenのダウンロードエラーを避けるために `wasm-pack build` に `--dev` オプションをつけて実行する。
  - `frontend/` 側テストを実行したい場合、事前に `engine/` 側でビルドしておく必要がある
