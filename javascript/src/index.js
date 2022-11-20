import { players, Reversi } from "./reversi";
import { renderBoard, initializeBoard } from "./draw";

const game = new Reversi();
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

    const legalPositions = game.getCurrentAllLegalPosition(players.first);
    if (legalPositions[i + 8 * j] > 0) {
      game.putAndReverse(i, j);
      draw(game.currentBoard(), i, j);

      while (true) {
        await sleep(500);

        const p = game.putAndReverseOpponent();
        if (!(p.length == 2 && p[0] >= 0 && p[1] >= 0)) {
          console.log(`putAndReverseOpponent returns invalid value: ${p}`);
          break;
        }
        [i, j] = p;
        draw(game.currentBoard(), i, j);
        if (game.hasPossibleMove(players.first)) {
          break;
        }
        if (!game.hasPossibleMove(players.second)) {
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

function draw(board, i, j) {
  const first = board.getBitboard(players.first);
  const second = board.getBitboard(players.second);
  renderBoard(first, second, i, j);
}
