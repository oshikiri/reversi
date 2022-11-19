import { Game, StrategyType, Player } from "reversi-wasm";
import { renderBoard, initializeBoard } from "./draw";

const game = Game.create(Player.First, StrategyType.NumdiskLookahead);
initializeBoard();
draw(game.currentBoard());

let boardLocked = false;
let i = -1;
let j = -1;
document.querySelectorAll(".cell").forEach((c) => {
  c.addEventListener("click", async () => {
    if (boardLocked) {
      return;
    }
    boardLocked = true;

    i = Number(c.dataset.boardColumn);
    j = Number(c.dataset.boardRow);

    const legalPositions = game.getCurrentAllLegalPosition(Player.First);
    if (legalPositions[i + 8 * j] > 0) {
      game.putAndReverse(i, j);
      draw(game.currentBoard(), i, j);

      while (true) {
        await sleep(500);

        const p = game.putAndReverseOpponent();
        if (!(p.length == 2 && p[0] >= 0 && p[1] >= 0)) {
          throw Error(`putAndReverseOpponent returns invalid value: ${p}`);
        }
        [i, j] = p;
        draw(game.currentBoard(), i, j);
        if (hasPossibleMove(game, Player.First)) {
          break;
        }
        if (!hasPossibleMove(game, Player.Second)) {
          break;
        }
      }
    }

    boardLocked = false;
  });
});

function hasPossibleMove(game, player) {
  const legalPositions = game.getCurrentAllLegalPosition(player);
  return legalPositions.reduce((l, r) => l + r) > 0;
}

const sleep = (milliSeconds) =>
  new Promise((resolve) => setTimeout(resolve, milliSeconds));

function draw(board, i, j) {
  const first = board.getBitboard(Player.First);
  const second = board.getBitboard(Player.Second);
  renderBoard(first, second, i, j);
}
