import { newBoard } from "reversi-wasm";

// TODO: Refactor constants
const BOARD_OFFSET = 10;
const N_COL_CELLS = 8;
const N_ROW_CELLS = 8;
const CELL_WIDTH = 30;
const BACKGROUND_WIDTH = 8 * (CELL_WIDTH + 1) + 2 * BOARD_OFFSET;
const DISK_RADIUS = 13;

const color = {
  background: "#0B610B",
  grid: "#000000",
  second: "#FFFFFF",
  first: "#000000",
};

const sleep = (milliSeconds) =>
  new Promise((resolve) => setTimeout(resolve, milliSeconds));

const drawBackground = (ctx) => {
  ctx.fillStyle = color.background;
  ctx.fillRect(0, 0, BACKGROUND_WIDTH, BACKGROUND_WIDTH);
  ctx.fill();
};

const drawGrid = (ctx) => {
  ctx.beginPath();
  ctx.strokeStyle = color.grid;

  // Vertical lines
  for (let i = 0; i <= N_COL_CELLS; i++) {
    const start = BOARD_OFFSET + i * (CELL_WIDTH + 1) + 1;
    const end = BOARD_OFFSET + (CELL_WIDTH + 1) * N_ROW_CELLS + 1;
    ctx.moveTo(start, BOARD_OFFSET);
    ctx.lineTo(start, end);
  }

  // Horizontal lines
  for (let j = 0; j <= N_ROW_CELLS; j++) {
    const start = BOARD_OFFSET + j * (CELL_WIDTH + 1) + 1;
    const end = BOARD_OFFSET + (CELL_WIDTH + 1) * N_COL_CELLS + 1;
    ctx.moveTo(BOARD_OFFSET, start);
    ctx.lineTo(end, start);
  }

  ctx.stroke();
};

const drawDisk = (ctx, i, j, color) => {
  const x = BOARD_OFFSET + (i + 1 / 2) * (CELL_WIDTH + 1) + 1;
  const y = BOARD_OFFSET + (j + 1 / 2) * (CELL_WIDTH + 1) + 1;

  ctx.beginPath();
  ctx.fillStyle = color;
  ctx.arc(x, y, DISK_RADIUS, 0, 2 * Math.PI);
  ctx.fill();
  ctx.stroke();
};

const drawDisks = (ctx, board) => {
  const first = board.getBitboard(false);
  const second = board.getBitboard(true);

  for (let k = 0; k < 64; k++) {
    const i = k % 8;
    const j = Math.floor(k / 8);

    if (first[k] == 1) {
      drawDisk(ctx, i, j, color.first);
    } else if (second[k] == 1) {
      drawDisk(ctx, i, j, color.second);
    }
  }
};

const convertToIdx = (x, y) => {
  x = x - BOARD_OFFSET;
  y = y - BOARD_OFFSET;
  const i = Math.floor(x / (CELL_WIDTH + 1));
  const j = Math.floor(y / (CELL_WIDTH + 1));

  if (0 <= Math.min(i, j) && Math.max(i, j) < 8) {
    return [i, j];
  } else {
    return undefined;
  }
};

const board = newBoard();
let legalPositions = board.getAllLegalPosition(false);

const canvas = document.getElementById("reversi-board");
canvas.height = "320";
canvas.width = "320";
if (canvas.getContext) {
  const context = canvas.getContext("2d");
  drawBackground(context);
  drawGrid(context);
  drawDisks(context, board);

  canvas.addEventListener("click", async function (clickEvent) {
    const idx = convertToIdx(clickEvent.offsetX, clickEvent.offsetY);
    if (idx) {
      const [i, j] = idx;
      if (legalPositions[i + 8 * j] == 0) return;
      board.putAndReverse(false, i, j);
      drawDisks(context, board);

      await sleep(500);

      const strategy = 0;
      board.putNextMove(true, strategy);

      drawDisks(context, board);
      legalPositions = board.getAllLegalPosition(false);
    }
  });
}
