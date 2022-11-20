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

    const i = Number(cell.dataset.boardColumn);
    const j = Number(cell.dataset.boardRow);

    if (game.isPossibleMove(players.first, i, j)) {
      game.putAndReverse(i, j);
      draw(game, i, j);

      while (true) {
        await sleep(500);

        const [suceed, i, j] = game.putAndReverseOpponent();
        if (!suceed) break;
        draw(game, i, j);
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

function draw(game, i, j) {
  const first = game.getCurrentBitBoard(players.first);
  const second = game.getCurrentBitBoard(players.second);
  renderBoard(first, second, i, j);
}
