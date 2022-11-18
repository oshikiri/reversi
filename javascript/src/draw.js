export function initializeBoard() {
  for (let r = 0; r < 8; r++) {
    for (let c = 0; c < 8; c++) {
      document.querySelector("#reversi-board").appendChild(createCell(r, c));
    }
  }
}

function createCell(r, c) {
  const cell = document.createElement("div");
  cell.classList = "cell";
  cell.classList.add("empty");
  cell.dataset.boardRow = r;
  cell.dataset.boardColumn = c;

  const diskFront = document.createElement("div");
  diskFront.classList = "disk-front";

  const diskBack = document.createElement("div");
  diskBack.classList = "disk-back";

  const disk = document.createElement("div");
  disk.classList = "disk";
  disk.appendChild(diskFront);
  disk.appendChild(diskBack);

  cell.appendChild(disk);

  return cell;
}

export function renderBoard(bitboardFirst, bitboardSecond, i, j, iPrev, jPrev) {
  if (iPrev >= 0 && jPrev >= 0) {
    activateCell(iPrev, jPrev, false);
  }
  drawDisks(bitboardFirst, bitboardSecond);
  if (i >= 0 && j >= 0) {
    activateCell(i, j, true);
  }
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

const drawDisks = (first, second) => {
  let firstScore = 0;
  let secondScore = 0;

  for (let k = 0; k < 64; k++) {
    const i = k % 8;
    const j = Math.floor(k / 8);

    if (first[k] == 1) {
      firstScore++;
      drawDisk(i, j, true);
    } else if (second[k] == 1) {
      secondScore++;
      drawDisk(i, j, false);
    }
  }
  document.querySelector("#scores").innerHTML = `${firstScore}-${secondScore}`;
};

const drawDisk = (i, j, isFirst) => {
  const cell = getCell(i, j);
  cell.classList.remove("empty");
  if (isFirst) {
    cell.classList.remove("flipped");
  } else {
    cell.classList.add("flipped");
  }
};
