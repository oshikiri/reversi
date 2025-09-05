export function initializeBoard() {
  for (let r = 0; r < 8; r++) {
    for (let c = 0; c < 8; c++) {
      document.getElementById("reversi-board").appendChild(createCell(r, c));
    }
  }
}

function createCell(r, c) {
  const disk = createDiv("disk");
  disk.appendChild(createDiv("disk-front"));
  disk.appendChild(createDiv("disk-back"));

  const cell = createDiv("cell", "empty");
  cell.dataset.boardRow = r;
  cell.dataset.boardColumn = c;
  cell.appendChild(disk);

  return cell;
}

function createDiv() {
  const div = document.createElement("div");
  for (const arg of arguments) {
    div.classList.add(String(arg));
  }
  return div;
}

export function renderBoard(bitboardFirst, bitboardSecond, i, j) {
  for (let r = 0; r < 8; r++) {
    for (let c = 0; c < 8; c++) {
      activateCell(r, c, r == i && c == j);
    }
  }
  updateDisks(bitboardFirst, bitboardSecond);
}

function activateCell(r, c, isActive) {
  const cell = getCell(r, c);
  if (isActive) {
    cell.classList.add("active");
  } else {
    cell.classList.remove("active");
  }
}

function getCell(r, c) {
  return document.querySelector(
    `[data-board-row="${r}"][data-board-column="${c}"]`,
  );
}

function updateDisks(firstBidboard, secondBitboard) {
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
  document.getElementById("scores").textContent =
    `${firstScore}-${secondScore}`;
}

function updateDiskIsFirst(r, c, isFirst) {
  const cell = getCell(r, c);
  cell.classList.remove("empty");
  if (isFirst) {
    cell.classList.remove("flipped");
  } else {
    cell.classList.add("flipped");
  }
}
