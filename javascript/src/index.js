import { players, Reversi } from "./reversi";
import { renderBoard, initializeBoard } from "./draw";

const game = new Reversi();
initializeBoard();
draw(game);

let boardLocked = false;
document.querySelectorAll(".cell").forEach((cell) => {
  cell.addEventListener("click", async () => {
    if (boardLocked) {
      return;
    }
    boardLocked = true;

    const c = Number(cell.dataset.boardColumn);
    const r = Number(cell.dataset.boardRow);

    if (game.isPossibleMove(players.first, r, c)) {
      game.putAndReverse(r, c);
      draw(game, r, c);

      while (true) {
        await sleep(500);

        const [ok, r, c] = game.putAndReverseOpponent();
        if (!ok) break;
        draw(game, r, c);
        const secondShouldPlayNextTurn =
          !game.hasPossibleMove(players.first) &&
          game.hasPossibleMove(players.second);
        if (!secondShouldPlayNextTurn) {
          break;
        }
      }
    }

    boardLocked = false;
  });
});

document.querySelector("#version").innerHTML = process.env.REVERSI_VERSION;

const sleep = (milliSeconds) =>
  new Promise((resolve) => setTimeout(resolve, milliSeconds));

function draw(game, r, c) {
  const first = game.getCurrentBitBoard(players.first);
  const second = game.getCurrentBitBoard(players.second);
  renderBoard(first, second, r, c);
}
