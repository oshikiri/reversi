import("./src/index.js").catch((e) =>
  console.error("Error importing `index.js`:", e)
);

document.querySelector("#version").innerHTML = process.env.REVERSI_VERSION;
