import { newBoard, Strategy } from "reversi-wasm";
import { sleep } from "./utils";
import { convertToIdx, drawBackground, drawDisks, drawGrid } from "./draw";

const board = newBoard();
let legalPositions = board.getAllLegalPosition(false);
let boardLocked = false;

const canvas = document.getElementById("reversi-board");
canvas.height = "960";
canvas.width = "960";
if (canvas.getContext) {
  const context = canvas.getContext("2d");
  drawBackground(context);
  drawGrid(context);
  drawDisks(context, board);

  canvas.addEventListener("click", async function (clickEvent) {
    if (boardLocked) {
      return;
    }
    boardLocked = true;

    const idx = convertToIdx(clickEvent.offsetX, clickEvent.offsetY);
    if (idx) {
      const [i, j] = idx;
      if (legalPositions[i + 8 * j] > 0) {
        board.putAndReverse(false, i, j);
        drawDisks(context, board);

        await sleep(500);

        board.putNextMove(true, Strategy.NumdiskLookahead1);

        drawDisks(context, board);
        legalPositions = board.getAllLegalPosition(false);
      }
    }

    boardLocked = false;
  });
}
