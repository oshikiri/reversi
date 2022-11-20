import { Game, StrategyType, Player } from "reversi-wasm";

export const players = { first: 0, second: 1 };

export class Reversi {
  constructor() {
    const player = Player.First;
    const strategy = StrategyType.NumdiskLookahead;
    this.game = Game.create(player, strategy);
  }
  currentBoard() {
    return this.game.currentBoard();
  }
  putAndReverse(i, j) {
    return this.game.putAndReverse(i, j);
  }
  putAndReverseOpponent() {
    return this.game.putAndReverseOpponent();
  }
  getCurrentAllLegalPosition(player) {
    return this.game.getCurrentAllLegalPosition(this.getPlayer(player));
  }
  hasPossibleMove(player) {
    const legalPositions = this.game.getCurrentAllLegalPosition(
      this.getPlayer(player)
    );
    return legalPositions.reduce((l, r) => l + r) > 0;
  }
  getPlayer(player) {
    return player == players.first ? Player.First : Player.Second;
  }
}
