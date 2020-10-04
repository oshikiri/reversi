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
      const [i, j] = idx;
      if (legalPositions[i + 8 * j] > 0) {
        game.putAndReverse(i, j);
        draw(game.currentBoard(), i, j);

        await sleep(500);

        const p = game.putAndReverseOpponent();
        draw(game.currentBoard(), p[0], p[1]);
        legalPositions = game.getCurrentAllLegalPosition(Player.First);
      }
    }

    boardLocked = false;
  });
}
