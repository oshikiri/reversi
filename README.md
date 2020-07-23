reversi
=====

[![GitHub Workflows Status](https://github.com/oshikiri/reversi/workflows/test/badge.svg)](https://github.com/oshikiri/reversi/actions?query=workflow%3A%22test%22)

Computer reversi on GitHub pages.


## TODO

- [ ] Setup CI
  - [x] prettier check
  - [ ] Implement tests
- [ ] Implement reversi board
  - [x] Setup canvas
  - [x] Handle click events
  - [ ] Implement data binding
- [ ] Setup wasm
- [ ] bidboard
  - [ ] Learn about bidboard
  - [ ] Implement bidboard
- [ ] Deploy to GitHub pages
- [ ] Implement algorithms
  - [ ] any-legal-move
    - [ ] Generate all possible legal moves
  - [ ] greedy
  - [ ] Survey on computer reversi algorithms


## References

- [Reversi \- Wikipedia](https://en.wikipedia.org/wiki/Reversi)
- bidboard
  - [オセロをビットボードで実装する \- Qiita](https://qiita.com/sensuikan1973/items/459b3e11d91f3cb37e43)
  - [ビットボードを用いた 4x4 オセロ 完全解析](http://vivi.dyndns.org/vivi/docs/puzzle/othello4x4.html)

## License
MIT

And this program based on the following repositories:

- <https://github.com/rustwasm/wasm_game_of_life>
