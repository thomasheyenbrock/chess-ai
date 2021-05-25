import * as fs from "fs";
import { Worker } from "worker_threads";
import { Result } from "../../move-generator";
import { CONCURRENT_GAMES, PAIRINGS_FILENAME } from "./constants";
import { Pairings } from "./types";

async function main() {
  const [, , generation] = process.argv;
  if (!generation) {
    throw new Error("generation missing");
  }

  const pairings: Pairings = JSON.parse(
    await fs.promises.readFile(PAIRINGS_FILENAME, "utf8")
  );

  let missing = Object.entries(pairings).filter(([, result]) => !result);

  while (missing.length > 0) {
    console.log(`${missing.length} games left`);
    const batch = missing.slice(0, CONCURRENT_GAMES).map(([id]) => id);

    const results = await Promise.all([
      new Promise<{ id: string; result: Result }[]>((resolve, reject) => {
        const worker = new Worker(`${__dirname}/worker.js`, {
          workerData: {
            batch: batch.slice(0, Math.floor(CONCURRENT_GAMES / 4)),
            generation,
          },
        });
        worker.on("message", resolve);
        worker.on("error", reject);
        worker.on("exit", (code) => {
          if (code !== 0)
            reject(new Error(`Worker stopped with exit code ${code}`));
        });
      }),
      new Promise<{ id: string; result: Result }[]>((resolve, reject) => {
        const worker = new Worker(`${__dirname}/worker.js`, {
          workerData: {
            batch: batch.slice(
              Math.floor(CONCURRENT_GAMES / 4),
              Math.floor(CONCURRENT_GAMES / 2)
            ),
            generation,
          },
        });
        worker.on("message", resolve);
        worker.on("error", reject);
        worker.on("exit", (code) => {
          if (code !== 0)
            reject(new Error(`Worker stopped with exit code ${code}`));
        });
      }),
      new Promise<{ id: string; result: Result }[]>((resolve, reject) => {
        const worker = new Worker(`${__dirname}/worker.js`, {
          workerData: {
            batch: batch.slice(
              Math.floor(CONCURRENT_GAMES / 2),
              Math.floor((CONCURRENT_GAMES / 4) * 3)
            ),
            generation,
          },
        });
        worker.on("message", resolve);
        worker.on("error", reject);
        worker.on("exit", (code) => {
          if (code !== 0)
            reject(new Error(`Worker stopped with exit code ${code}`));
        });
      }),
      new Promise<{ id: string; result: Result }[]>((resolve, reject) => {
        const worker = new Worker(`${__dirname}/worker.js`, {
          workerData: {
            batch: batch.slice(Math.floor((CONCURRENT_GAMES / 4) * 3)),
            generation,
          },
        });
        worker.on("message", resolve);
        worker.on("error", reject);
        worker.on("exit", (code) => {
          if (code !== 0)
            reject(new Error(`Worker stopped with exit code ${code}`));
        });
      }),
    ]);

    results.flat().map(({ id, result }) => {
      pairings[id] = result;
    });

    await fs.promises.writeFile(PAIRINGS_FILENAME, JSON.stringify(pairings));
    missing = Object.entries(pairings).filter(([, result]) => !result);
  }
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
