import * as tf from "@tensorflow/tfjs-node";
import * as fs from "fs";
import * as path from "path";
import { Player, Result } from "../../move-generator";
import { NUMBER_OF_NETWORKS, REPRODUCTION, SURVIVORS } from "./constants";
import { parsePairings, stringifyPairings } from "./pairings";
import { Pairings } from "./types";

function generateRandomNetwork() {
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

  console.log(`Generating networks for generation ${generation}`);

  const networkIds: string[] = [];

  const base = path.resolve(__dirname, `generation${generation}`);
  await fs.promises.rm(base, { recursive: true, force: true });
  await fs.promises.mkdir(base);

  if (generation > 1) {
    console.log("Evaluate scores for previous generaton");
    const pairingFiles = await fs.promises.readdir(
      path.join(__dirname, `generation${generation - 1}`)
    );
    const pairings: Pairings[] = await Promise.all(
      pairingFiles
        .filter((filename) => filename.match(/^pairings.*\.txt$/))
        .map(async (filename) =>
          parsePairings(
            await fs.promises.readFile(
              path.join(__dirname, `generation${generation - 1}`, filename),
              "utf8"
            )
          )
        )
    );
    const scores = Object.entries(
      pairings.reduce<Pairings>((acc, pairing) => ({ ...acc, ...pairing }), {})
    ).reduce<Score>((acc, [pairingId, game]) => {
      if (!game || !game.result) {
        throw new Error(`no result for pairing ${pairingId}`);
      }

      const [white, black] = pairingId.split("-");
      acc[white] = acc[white] || 0;
      acc[black] = acc[black] || 0;

      const halfMoves =
        (game.moveCounter - 1) * 2 + (game.player === Player.WHITE ? 0 : 1);
      const moveScore = 10 * 2 ** (-halfMoves / 100);

      switch (game.result) {
        case Result.WHITE:
          acc[white] += 5 + moveScore;
          break;
        case Result.BLACK:
          acc[black] += 5 + moveScore;
          break;
        case Result.STALEMATE:
        case Result.DEAD_POSITION:
          acc[white] += 1 + moveScore / 2;
          acc[black] += 1 + moveScore / 2;
          break;
        case Result.REPITITION:
        case Result.FIFTY_MOVE_RULE:
          acc[white] -= 2;
          acc[black] -= 2;
          break;
      }
      return acc;
    }, {});
    console.log(scores);

    console.log("Determining survivors");
    const survivorIds = Object.entries(scores)
      .sort(([, a], [, b]) => b - a)
      .slice(0, SURVIVORS)
      .map(([id]) => id);
    console.log(survivorIds);

    for (let i = 0; i < survivorIds.length; i++) {
      console.log(`Mutating best networks: ${i} / ${survivorIds.length}`);
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
        console.log(`  Cloning network: ${j} / ${REPRODUCTION}`);
        const newModel = generateRandomNetwork();
        newModel.setWeights(
          weights.map((weight) =>
            weight.add(tf.randomUniform(weight.shape, -0.5, 0.5))
          )
        );
        await newModel.save(`file://${base}/${survivorIds[i]}_${j}`);
        networkIds.push(`${survivorIds[i]}_${j}`);
      }
    }
  }

  const newNetworks =
    NUMBER_OF_NETWORKS - (generation > 1 ? SURVIVORS * (REPRODUCTION + 1) : 0);

  for (let i = 0; i < newNetworks; i++) {
    console.log(`Generating random networks: ${i} / ${newNetworks}`);
    const network = generateRandomNetwork();
    await network.save(`file://${base}/${i}`);
    networkIds.push(i.toString());
  }

  const pairings: Pairings[] = [{}];
  for (let i = 0; i < networkIds.length - 1; i++) {
    for (let j = i + 1; j < networkIds.length; j++) {
      if (
        Object.keys(pairings[pairings.length - 1]).length >=
        (networkIds.length ** 2 - networkIds.length) / 4
      ) {
        pairings.push({});
      }
      pairings[pairings.length - 1][`${networkIds[i]}-${networkIds[j]}`] = null;
      pairings[pairings.length - 1][`${networkIds[j]}-${networkIds[i]}`] = null;
    }
  }

  await Promise.all(
    pairings.map((p, i) =>
      fs.promises.writeFile(
        path.join(
          __dirname,
          `generation${process.argv[2]}`,
          `pairings_${i}.txt`
        ),
        stringifyPairings(p)
      )
    )
  );
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
