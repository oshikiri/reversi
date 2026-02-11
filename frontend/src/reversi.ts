import { Game, StrategyType, Player } from "reversi-wasm";

export const players = { first: 0, second: 1 } as const;

type PlayerId = (typeof players)[keyof typeof players];

type MoveResult = [boolean, number, number];

export class Reversi {
  #game: Game;

  constructor() {
    const player = Player.First;
    const strategy = StrategyType.NumdiskLookahead;
    this.#game = Game.create(player, strategy);
  }
  getCurrentBitBoard(player: PlayerId): number[] {
    const board = this.#game.currentBoard();
    return board.getBitboard(this.#getPlayer(player));
  }
  putAndReverse(r: number, c: number): void {
    this.#game.putAndReverse(c, r);
  }
  putAndReverseOpponent(): MoveResult {
    const p = this.#game.putAndReverseOpponent();
    if (p.length == 2 && p[0] >= 0 && p[1] >= 0) {
      return [true, p[1], p[0]];
    }
    console.log(`putAndReverseOpponent returns invalid value: ${p}`);
    return [false, -1, -1];
  }
  isPossibleMove(player: PlayerId, r: number, c: number): boolean {
    const legalPositions = this.#getCurrentAllLegalPosition(player);
    return legalPositions[c + 8 * r] > 0;
  }
  hasPossibleMove(player: PlayerId): boolean {
    const legalPositions = this.#getCurrentAllLegalPosition(player);
    return legalPositions.reduce((l, r) => l + r) > 0;
  }
  #getCurrentAllLegalPosition(player: PlayerId): number[] {
    return this.#game.getCurrentAllLegalPosition(this.#getPlayer(player));
  }
  #getPlayer(player: PlayerId): Player {
    return player == players.first ? Player.First : Player.Second;
  }
}
