const concurrently = require("concurrently");
const fs = require("fs");
const path = require("path");

const GAMES = parseInt(process.argv[2], 10);
const CONCURRENCY = parseInt(process.argv[3], 10);

if (GAMES % CONCURRENCY !== 0) {
  throw new Error(
    "The number of concurrent games has to divide the total number of games evenly."
  );
}

const GAMES_PER_PROCESS = GAMES / CONCURRENCY;

async function main() {
  while (true) {
    const commands = [];
    const indices = [];
    for (let i = 0; i < CONCURRENCY; i++) {
      commands.push(
        process.platform === "win32"
          ? `call ${path.join(__dirname, "mcts.exe")} ${i} ${GAMES_PER_PROCESS}`
          : `${path.join(__dirname, "mcts")} ${i} ${GAMES_PER_PROCESS}`
      );
      indices.push(i);
    }
    await concurrently(commands);
    await concurrently([
      `${path.join(__dirname, "train")} ${indices.join(" ")}`,
    ]);
    for (let i = 0; i <= GAMES; i++) {
      await fs.promises.rm(path.join(__dirname, `value.${i}.txt`));
      await fs.promises.rm(path.join(__dirname, `policy.${i}.txt`));
    }
  }
}

main();
