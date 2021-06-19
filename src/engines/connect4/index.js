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
  let counter = parseInt(process.argv[4], 10) || 0;
  while (true) {
    counter += 1;
    console.log("================================");
    console.log(`Step ${counter}`);
    console.log("================================");

    const commands = [];
    const indices = [];
    for (let i = 0; i < CONCURRENCY; i++) {
      commands.push(
        process.platform === "win32"
          ? `call ${path.join(
              __dirname,
              "self_play.exe"
            )} ${i} ${GAMES_PER_PROCESS}`
          : `${path.join(__dirname, "self_play")} ${i} ${GAMES_PER_PROCESS}`
      );
      indices.push(i);
    }
    await concurrently(commands);
    await concurrently([
      `${path.join(__dirname, "train")} ${indices.join(" ")}`,
    ]);

    console.log("Commiting weights and biases");
    await concurrently([`git add value.fc1.weight`]);
    await concurrently([`git add value.fc1.bias`]);
    await concurrently([`git add value.fc2.weight`]);
    await concurrently([`git add value.fc2.bias`]);
    await concurrently([`git add value.fc3.weight`]);
    await concurrently([`git add value.fc3.bias`]);
    await concurrently([`git add value.fc4.weight`]);
    await concurrently([`git add value.fc4.bias`]);
    await concurrently([`git add policy.fc1.weight`]);
    await concurrently([`git add policy.fc1.bias`]);
    await concurrently([`git add policy.fc2.weight`]);
    await concurrently([`git add policy.fc2.bias`]);
    await concurrently([`git add policy.fc3.weight`]);
    await concurrently([`git add policy.fc3.bias`]);
    await concurrently([`git add policy.fc4.weight`]);
    await concurrently([`git add policy.fc4.bias`]);
    await concurrently([`git commit -m "step ${counter}"`]);

    for (let i = 0; i < CONCURRENCY; i++) {
      await fs.promises.rm(path.join(__dirname, `value.${i}.txt`));
      await fs.promises.rm(path.join(__dirname, `policy.${i}.txt`));
    }
  }
}

main();
