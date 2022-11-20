import { Game, StrategyType, Player } from "reversi-wasm";

export const players = { first: 0, second: 1 };

export class Reversi {
  constructor() {
    const player = Player.First;
    const strategy = StrategyType.NumdiskLookahead;
    this.game = Game.create(player, strategy);
  }
  getCurrentBitBoard(player) {
    const board = this.game.currentBoard();
    return board.getBitboard(this.#getPlayer(player));
  }
  putAndReverse(i, j) {
    return this.game.putAndReverse(i, j);
  }
  putAndReverseOpponent() {
    const p = this.game.putAndReverseOpponent();
    if (p.length == 2 && p[0] >= 0 && p[1] >= 0) {
      return [true, p[0], p[1]];
    }
    console.log(`putAndReverseOpponent returns invalid value: ${p}`);
    return false, -1, -1;
  }
  isPossibleMove(player, i, j) {
    const legalPositions = this.#getCurrentAllLegalPosition(player);
    return legalPositions[i + 8 * j] > 0;
  }
  hasPossibleMove(player) {
    const legalPositions = this.#getCurrentAllLegalPosition(player);
    return legalPositions.reduce((l, r) => l + r) > 0;
  }
  #getCurrentAllLegalPosition(player) {
    return this.game.getCurrentAllLegalPosition(this.#getPlayer(player));
  }
  #getPlayer(player) {
    return player == players.first ? Player.First : Player.Second;
  }
}
