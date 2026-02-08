export function initializeBoard(): void {
  for (let r = 0; r < 8; r++) {
    for (let c = 0; c < 8; c++) {
      document.getElementById("reversi-board")?.appendChild(createCell(r, c));
    }
  }
}

function createCell(r: number, c: number): HTMLDivElement {
  const disk = createDiv("disk");
  disk.appendChild(createDiv("disk-front"));
  disk.appendChild(createDiv("disk-back"));

  const cell = createDiv("cell", "empty");
  cell.dataset.boardRow = String(r);
  cell.dataset.boardColumn = String(c);
  cell.appendChild(disk);

  return cell;
}

function createDiv(...classes: Array<string | number>): HTMLDivElement {
  const div = document.createElement("div");
  for (const arg of classes) {
    div.classList.add(String(arg));
  }
  return div;
}

export function renderBoard(
  bitboardFirst: ArrayLike<number>,
  bitboardSecond: ArrayLike<number>,
  i?: number,
  j?: number,
): void {
  for (let r = 0; r < 8; r++) {
    for (let c = 0; c < 8; c++) {
      activateCell(r, c, r == i && c == j);
    }
  }
  updateDisks(bitboardFirst, bitboardSecond);
}

function activateCell(r: number, c: number, isActive: boolean): void {
  const cell = getCell(r, c);
  if (isActive) {
    cell.classList.add("active");
  } else {
    cell.classList.remove("active");
  }
}

function getCell(r: number, c: number): HTMLElement {
  const cell = document.querySelector<HTMLElement>(
    `[data-board-row="${r}"][data-board-column="${c}"]`,
  );
  if (!cell) {
    throw new Error(`Cell not found: ${r}, ${c}`);
  }
  return cell;
}

function updateDisks(
  firstBidboard: ArrayLike<number>,
  secondBitboard: ArrayLike<number>,
): void {
  let firstScore = 0;
  let secondScore = 0;

  for (let r = 0; r < 8; r++) {
    for (let c = 0; c < 8; c++) {
      const k = 8 * r + c;
      if (firstBidboard[k] == 1) {
        firstScore++;
        updateDiskIsFirst(r, c, true);
      } else if (secondBitboard[k] == 1) {
        secondScore++;
        updateDiskIsFirst(r, c, false);
      }
    }
  }
  const scores = document.getElementById("scores");
  if (scores) {
    scores.textContent = `${firstScore}-${secondScore}`;
  }
}

function updateDiskIsFirst(r: number, c: number, isFirst: boolean): void {
  const cell = getCell(r, c);
  cell.classList.remove("empty");
  if (isFirst) {
    cell.classList.remove("flipped");
  } else {
    cell.classList.add("flipped");
  }
}
