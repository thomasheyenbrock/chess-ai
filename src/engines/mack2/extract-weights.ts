import * as tf from "@tensorflow/tfjs-node";
import * as fs from "fs";
import * as path from "path";

async function extractWeights() {
  const [, , generation, id] = process.argv;
  const network = await tf.loadLayersModel(
    `file://${path.resolve(
      __dirname,
      `generation${generation}`,
      id,
      "model.json"
    )}`
  );
  const weights = network.getWeights().map((weight) => ({
    shape: weight.shape,
    data: Array.from(weight.dataSync()),
  }));
  await fs.promises.writeFile(
    path.resolve(__dirname, "weights.json"),
    JSON.stringify(weights)
  );
}

extractWeights();
