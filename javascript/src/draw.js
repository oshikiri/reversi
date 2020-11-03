import { Player } from "reversi-wasm";

// TODO: Refactor constants
const BOARD_OFFSET = 30;
const GRID_WIDTH = 3;
const N_COL_CELLS = 8;
const N_ROW_CELLS = 8;
const CELL_WIDTH = 90;
export const BACKGROUND_WIDTH = 8 * (CELL_WIDTH + 1) + 2 * BOARD_OFFSET;
const DISK_RADIUS = 39;

const color = {
  background: "#0B610B",
  grid: "#000000",
  second: "#FFFFFF",
  first: "#000000",
};

export const drawBackground = (ctx) => {
  ctx.fillStyle = color.background;
  ctx.fillRect(0, 0, BACKGROUND_WIDTH, BACKGROUND_WIDTH);
  ctx.fill();
};

export const drawGrid = (ctx) => {
  ctx.beginPath();
  ctx.strokeStyle = color.grid;
  ctx.lineWidth = GRID_WIDTH;

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
  drawCircle(ctx, i, j, color, DISK_RADIUS);
};

export const drawCircle = (ctx, i, j, color, radius) => {
  const x = BOARD_OFFSET + (i + 1 / 2) * (CELL_WIDTH + 1) + 1;
  const y = BOARD_OFFSET + (j + 1 / 2) * (CELL_WIDTH + 1) + 1;

  ctx.beginPath();
  ctx.fillStyle = color;
  ctx.arc(x, y, radius, 0, 2 * Math.PI);
  ctx.fill();
};

export const drawDisks = (ctx, board) => {
  const first = board.getBitboard(Player.First);
  const second = board.getBitboard(Player.Second);

  let firstScore = 0;
  let secondScore = 0;

  for (let k = 0; k < 64; k++) {
    const i = k % 8;
    const j = Math.floor(k / 8);

    if (first[k] == 1) {
      firstScore++;
      drawDisk(ctx, i, j, color.first);
    } else if (second[k] == 1) {
      secondScore++;
      drawDisk(ctx, i, j, color.second);
    }
  }
  document.querySelector("#scores").innerHTML = `${firstScore}-${secondScore}`;
};

export const convertToIdx = (x, y) => {
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
