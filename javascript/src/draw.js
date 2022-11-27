export function initializeBoard() {
  for (let r = 0; r < 8; r++) {
    for (let c = 0; c < 8; c++) {
      document.querySelector("#reversi-board").appendChild(createCell(r, c));
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
  for (let k = 0; k < 64; k++) {
    const r = Math.floor(k / 8);
    const c = k % 8;
    activateCell(r, c, r == i && c == j);
  }
  updateDisks(bitboardFirst, bitboardSecond);
}

function activateCell(i, j, isActive) {
  const cell = getCell(i, j);
  if (isActive) {
    cell.classList.add("active");
  } else {
    cell.classList.remove("active");
  }
}

function getCell(i, j) {
  return document.querySelector(
    `[data-board-row="${j}"][data-board-column="${i}"]`
  );
}

function updateDisks(firstBidboard, secondBitboard) {
  let firstScore = 0;
  let secondScore = 0;

  for (let k = 0; k < 64; k++) {
    const i = k % 8;
    const j = Math.floor(k / 8);

    if (firstBidboard[k] == 1) {
      firstScore++;
      updateDiskIsFirst(i, j, true);
    } else if (secondBitboard[k] == 1) {
      secondScore++;
      updateDiskIsFirst(i, j, false);
    }
  }
  document.querySelector("#scores").innerHTML = `${firstScore}-${secondScore}`;
}

function updateDiskIsFirst(i, j, isFirst) {
  const cell = getCell(i, j);
  cell.classList.remove("empty");
  if (isFirst) {
    cell.classList.remove("flipped");
  } else {
    cell.classList.add("flipped");
  }
}
