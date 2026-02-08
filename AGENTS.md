# Repository Guidelines

- 実装完了後、私に報告する前に下記を確認すること
  - `frontend/` 以下を変更した場合は、 `npm run check:format` `npm test` `npm run build`
  - `engine/` 以下を変更した場合は、 `cargo test` `cargo clippy -- -D warnings` `wasm-pack build`
- PRを作成する際、PRタイトルは git-commit スキルの指定に従うこと
