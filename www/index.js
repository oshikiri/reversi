const BOARD_WIDTH = 1000;
const BOARD_OFFSET = 10;
const width = 8;
const height = 8;
const CELL_SIZE = 20;
const DISK_RADIUS = 10;
const color = {
  background: "#0B610B",
  grid: "#000000",
  white: "#FFFFFF",
  black: "#000000",
};

const drawBackground = (ctx) => {
  ctx.fillStyle = color.background;
  ctx.fillRect(BOARD_OFFSET, BOARD_OFFSET, BOARD_WIDTH, BOARD_WIDTH);
  ctx.fill();
};

const drawGrid = (ctx) => {
  ctx.beginPath();
  ctx.strokeStyle = color.grid;

  // Vertical lines
  const verticalOffset = BOARD_OFFSET + 10;
  for (let i = 0; i <= width; i++) {
    const start = verticalOffset + i * (CELL_SIZE + 1) + 1;
    const end = verticalOffset + (CELL_SIZE + 1) * height + 1;
    ctx.moveTo(start, verticalOffset);
    ctx.lineTo(start, end);
  }

  // Horizontal lines
  const horizontalOffset = BOARD_OFFSET + 10;
  for (let j = 0; j <= height; j++) {
    const start = horizontalOffset + j * (CELL_SIZE + 1) + 1;
    const end = horizontalOffset + (CELL_SIZE + 1) * width + 1;
    ctx.moveTo(horizontalOffset, start);
    ctx.lineTo(end, start);
  }

  // TODO: dots

  ctx.stroke();
};

const drawDisk = (ctx, x, y, color) => {
  ctx.beginPath();
  ctx.fillStyle = color;
  ctx.arc(x, y, DISK_RADIUS, 0, Math.PI * 2);
  ctx.fill();
  ctx.stroke();
};

const drawDisks = (ctx) => {
  (x = 50), (y = 50);
  drawDisk(ctx, x, y, color.white);
  drawDisk(ctx, x + 50, y + 50, color.black);
};

const canvas = document.getElementById("reversi-board");
if (canvas.getContext) {
  const context = canvas.getContext("2d");
  drawBackground(context);
  drawGrid(context);
  drawDisks(context);
}
