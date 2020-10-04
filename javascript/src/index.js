import { newBoard, StrategyType, Player } from "reversi-wasm";
import { sleep } from "./utils";
import { convertToIdx, drawBackground, drawCircle, drawDisks, drawGrid } from "./draw";

const board = newBoard();
let legalPositions = board.getAllLegalPosition(Player.First);
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
  draw(board);

  canvas.addEventListener("click", async function (clickEvent) {
    if (boardLocked) {
      return;
    }
    boardLocked = true;

    const idx = convertToIdx(clickEvent.offsetX, clickEvent.offsetY);
    if (idx) {
      const [i, j] = idx;
      if (legalPositions[i + 8 * j] > 0) {
        board.putAndReverse(Player.First, i, j);
        draw(board, i, j);

        await sleep(500);

        const p = board.putNextMove(Player.Second, StrategyType.NumdiskLookahead);

        draw(board, p[0], p[1]);
        legalPositions = board.getAllLegalPosition(Player.First);
      }
    }

    boardLocked = false;
  });
}
