import { Game } from "../../move-generator";
import { moveForOutputIndex } from "./output-layer-mapping";

export function chooseMove(
  game: Game,
  output: Float32Array | Int32Array | Uint8Array | number[]
) {
  let max = -Infinity;
  const validGames: { score: number; game: Game }[] = [];
  for (let i = 0; i < output.length; i++) {
    let possibleGame = game.possibleMoves[moveForOutputIndex[i]];
    if (possibleGame) {
      const score = output[i];
      validGames.push({ score, game: possibleGame });
      max = Math.max(max, score);
    }
  }

  const topGames = validGames
    .filter((v) => v.score >= 0.95 * max)
    .sort((g1, g2) => g1.score - g2.score)
    .slice(0, 3);

  return topGames[Math.floor(Math.random() * topGames.length)].game;
}
