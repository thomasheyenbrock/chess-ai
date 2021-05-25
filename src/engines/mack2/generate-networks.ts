import * as tf from "@tensorflow/tfjs-node";
import * as fs from "fs";
import * as path from "path";
import { Result } from "../../move-generator";
import {
  NUMBER_OF_NETWORKS,
  PAIRINGS_FILENAME,
  REPRODUCTION,
  SURVIVORS,
} from "./constants";
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

type Score = { [networkId: string]: number };

async function main() {
  const generation = parseInt(process.argv[2], 10);
  if (!generation) {
    throw new Error("generation missing");
  }

  const networkIds: string[] = [];

  const base = path.resolve(__dirname, `generation${generation}`);
  await fs.promises.rm(base, { recursive: true, force: true });
  await fs.promises.mkdir(base);

  if (generation > 1) {
    const pairings: Pairings = JSON.parse(
      await fs.promises.readFile(
        path.join(__dirname, `generation${generation - 1}`, "pairings.json"),
        "utf8"
      )
    );
    const scores = Object.entries(pairings).reduce<Score>(
      (acc, [pairingId, result]) => {
        const [white, black] = pairingId.split("-");
        acc[white] = acc[white] || 0;
        acc[black] = acc[black] || 0;
        switch (result) {
          case null:
            throw new Error(`no result for pairing ${pairingId}`);
          case Result.WHITE:
            acc[white] += 10;
            break;
          case Result.BLACK:
            acc[black] += 10;
            break;
          case Result.STALEMATE:
          case Result.DEAD_POSITION:
            acc[white] += 5;
            acc[black] += 5;
            break;
          case Result.REPITITION:
          case Result.FIFTY_MOVE_RULE:
            acc[white] += 1;
            acc[black] += 1;
            break;
        }
        return acc;
      },
      {}
    );
    const survivorIds = Object.entries(scores)
      .sort(([, a], [, b]) => b - a)
      .slice(0, SURVIVORS)
      .map(([id]) => id);

    for (let i = 0; i < survivorIds.length; i++) {
      console.log("Mutating best networks", i / survivorIds.length);
      const model = await tf.loadLayersModel(
        `file://${path.resolve(
          __dirname,
          `generation${generation - 1}`,
          survivorIds[i],
          "model.json"
        )}`
      );
      await model.save(`file://${base}/${survivorIds[i]}_0`);
      networkIds.push(`${survivorIds[i]}_0`);

      const weights = model.getWeights();
      for (let j = 1; j <= REPRODUCTION; j++) {
        console.log(
          "Mutating best networks",
          i / survivorIds.length + j / survivorIds.length / REPRODUCTION
        );
        const newModel = await generateRandomNetwork();
        newModel.setWeights(
          weights.map((weight) =>
            weight.add(tf.randomUniform(weight.shape, -0.5, 0.5))
          )
        );
        await newModel.save(`file://${base}/${survivorIds[i]}_${j}`);
        networkIds.push(`${survivorIds[i]}_${j}`);
      }
    }
    console.log("Mutating best networks", 1);
  }

  const newNetworks =
    NUMBER_OF_NETWORKS - (generation > 1 ? SURVIVORS * (REPRODUCTION + 1) : 0);

  for (let i = 0; i < newNetworks; i++) {
    console.log("Generating random networks", i / newNetworks);
    const network = await generateRandomNetwork();
    await network.save(`file://${base}/${i}`);
    networkIds.push(i.toString());
  }
  console.log("Generating random networks", 1);

  const pairings: Pairings = {};
  for (let i = 0; i < networkIds.length - 1; i++) {
    for (let j = i + 1; j < networkIds.length; j++) {
      pairings[`${networkIds[i]}-${networkIds[j]}`] = null;
      pairings[`${networkIds[j]}-${networkIds[i]}`] = null;
    }
  }

  await fs.promises.writeFile(PAIRINGS_FILENAME, JSON.stringify(pairings));
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});