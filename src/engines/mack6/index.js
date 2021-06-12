const concurrently = require("concurrently");
const fs = require("fs");
const path = require("path");

const GAMES = parseInt(process.argv[2], 10);
const CONCURRENRY = parseInt(process.argv[3], 10);

async function main() {
  while (true) {
    for (let i = 0; i < GAMES / CONCURRENRY; i++) {
      console.log(
        CONCURRENRY === 1
          ? `Playing game ${i * CONCURRENRY}`
          : `Playing games ${i * CONCURRENRY} to ${(i + 1) * CONCURRENRY - 1}`
      );
      const commands = [];
      for (let j = 0; j < CONCURRENRY; j++) {
        commands.push(`${path.join(__dirname, "mcts")} ${i + j}`);
      }
      await concurrently(commands);
    }
    const indices = Array.from({ length: GAMES })
      .map((_, i) => i)
      .join(" ");
    await concurrently([`${path.join(__dirname, "train")} ${indices}`]);
    for (let i = 0; i <= GAMES; i++) {
      await fs.promises.rm(path.join(__dirname, `value.${i}.txt`));
      await fs.promises.rm(path.join(__dirname, `policy.${i}.txt`));
    }
  }
}

main();
