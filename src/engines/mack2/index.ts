import * as tf from "@tensorflow/tfjs";
import { PickMove } from "../type";
import { getInputForGame } from "./input";
import { chooseMove } from "./output";
import weights from "./weights.json";

const network = tf.sequential({
  layers: [
    tf.layers.dense({
      inputShape: [837],
      units: 1405,
      activation: "sigmoid",
    }),
    tf.layers.dense({ units: 1972, activation: "sigmoid" }),
  ],
});

network.setWeights(
  (weights as { data: number[]; shape: number[] }[]).map(({ data, shape }) =>
    tf.tensor(data, shape)
  )
);

const pickMove: PickMove = function (game) {
  const input = tf.tensor(getInputForGame(game), [1, 837]);

  const output = (network.predict(input) as tf.Tensor).dataSync();

  return chooseMove(game, output);
};

export default pickMove;
