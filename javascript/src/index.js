import { Game, StrategyType, Player } from "reversi-wasm";
import { sleep } from "./utils";
import {
  convertToIdx,
  drawBackground,
  drawCircle,
  drawDisks,
  drawGrid,
} from "./draw";

const game = Game.create(Player.First, StrategyType.NumdiskLookahead);
let legalPositions = game.getCurrentAllLegalPosition(Player.First);
let boardLocked = false;

const canvas = document.getElementById("reversi-board");
canvas.height = "960";
canvas.width = "960";
if (canvas.getContext) {
  const context = canvas.getContext("2d");

  const draw = (board, i, j) => {
    drawBackground(context);
    drawGrid(context);
    drawDisks(context, board);
    if (i && j) {
      drawCircle(context, i, j, "red", 5);
    }
  };
  draw(game.currentBoard());

  canvas.addEventListener("click", async function (clickEvent) {
    if (boardLocked) {
      return;
    }
    boardLocked = true;

    const idx = convertToIdx(clickEvent.offsetX, clickEvent.offsetY);
    if (idx) {
      let [i, j] = idx;
      if (legalPositions[i + 8 * j] > 0) {
        game.putAndReverse(i, j);
        draw(game.currentBoard(), i, j);

        while (true) {
          await sleep(500);

          const p = game.putAndReverseOpponent();
          if (p[0] && p[1]) {
            i = p[0];
            j = p[1];
          }
          draw(game.currentBoard(), i, j);
          legalPositions = game.getCurrentAllLegalPosition(Player.First);
          if (legalPositions.reduce((l, r) => l + r) > 0) {
            break;
          }

          const legalPositionsOpponent = game.getCurrentAllLegalPosition(Player.Second);
          if (legalPositionsOpponent.reduce((l, r) => l + r) == 0) {
            break;
          }
        }
      }
    }

    boardLocked = false;
  });
}
