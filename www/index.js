// TODO: Refactor constants
const BOARD_OFFSET = 10;
const width = 8;
const height = 8;
const CELL_SIZE = 30;
const BOARD_WIDTH = 8 * (CELL_SIZE + 1) + 2 * (BOARD_OFFSET + 10);
const DISK_RADIUS = 13;
const horizontalOffset = BOARD_OFFSET + 10;
const verticalOffset = BOARD_OFFSET + 10;

const color = {
  background: "#0B610B",
  grid: "#000000",
  white: "#FFFFFF",
  black: "#000000",
};

const drawBackground = (ctx) => {
  ctx.fillStyle = color.background;
  ctx.fillRect(0, 0, BOARD_WIDTH, BOARD_WIDTH);
  ctx.fill();
};

const drawGrid = (ctx) => {
  ctx.beginPath();
  ctx.strokeStyle = color.grid;

  // Vertical lines
  for (let i = 0; i <= width; i++) {
    const start = verticalOffset + i * (CELL_SIZE + 1) + 1;
    const end = verticalOffset + (CELL_SIZE + 1) * height + 1;
    ctx.moveTo(start, verticalOffset);
    ctx.lineTo(start, end);
  }

  // Horizontal lines
  for (let j = 0; j <= height; j++) {
    const start = horizontalOffset + j * (CELL_SIZE + 1) + 1;
    const end = horizontalOffset + (CELL_SIZE + 1) * width + 1;
    ctx.moveTo(horizontalOffset, start);
    ctx.lineTo(end, start);
  }

  ctx.stroke();
};

const drawDisk = (ctx, i, j, color) => {
  const x = horizontalOffset + (i + 1 / 2) * (CELL_SIZE + 1) + 1;
  const y = verticalOffset + (j + 1 / 2) * (CELL_SIZE + 1) + 1;

  ctx.beginPath();
  ctx.fillStyle = color;
  ctx.arc(x, y, DISK_RADIUS, 0, Math.PI * 2);
  ctx.fill();
  ctx.stroke();
};

const drawDisks = (ctx) => {
  // Initial position
  drawDisk(ctx, 3, 3, color.white);
  drawDisk(ctx, 4, 4, color.white);
  drawDisk(ctx, 3, 4, color.black);
  drawDisk(ctx, 4, 3, color.black);
};

const canvas = document.getElementById("reversi-board");
canvas.height = "320";
canvas.width = "320";
if (canvas.getContext) {
  const context = canvas.getContext("2d");
  drawBackground(context);
  drawGrid(context);
  drawDisks(context);
}
