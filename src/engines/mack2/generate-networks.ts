import * as tf from "@tensorflow/tfjs-node";
import * as fs from "fs";
import * as path from "path";
import { NUMBER_OF_NETWORKS, PAIRINGS_FILENAME } from "./constants";
import { Pairings } from "./types";

async function generateRandomNetwork() {
  return tf.sequential({
    layers: [
      tf.layers.dense({
        inputShape: [837],
        units: 1405,
        activation: "sigmoid",
      }),
      tf.layers.dense({ units: 1972, activation: "sigmoid" }),
    ],
  });
}

async function main() {
  for (let i = 0; i < NUMBER_OF_NETWORKS; i++) {
    console.log("Generating random networks", i / NUMBER_OF_NETWORKS);
    const dirname = path.resolve(__dirname, "networks", i.toString());
    await fs.promises.rm(dirname, { recursive: true, force: true });

    const network = await generateRandomNetwork();
    await network.save(`file://${dirname}`);
  }
  console.log("Generating random networks", 1);

  const pairings: Pairings = {};
  for (let i = 0; i < NUMBER_OF_NETWORKS - 1; i++) {
    for (let j = i + 1; j < NUMBER_OF_NETWORKS; j++) {
      pairings[`${i}-${j}`] = null;
      pairings[`${j}-${i}`] = null;
    }
  }

  await fs.promises.writeFile(PAIRINGS_FILENAME, JSON.stringify(pairings));
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
