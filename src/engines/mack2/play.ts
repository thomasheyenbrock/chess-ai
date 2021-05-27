import * as tf from "@tensorflow/tfjs-node";
import * as fs from "fs";
import * as path from "path";
import { gameFromFen } from "../../fen";
import { Game, Player, setGameResult } from "../../move-generator";
import { MOVES_PER_PLAY } from "./constants";
import { getInputForGame } from "./input";
import { chooseMove } from "./output";
import { parsePairings, stringifyPairings } from "./pairings";
import { Pairings } from "./types";

type GameWithPlayers = {
  game: Game;
  whiteId: string;
  blackId: string;
};

type GamesPerNetwork = {
  [networkId: string]: GameWithPlayers[];
};

function pickMoves(games: GameWithPlayers[], network: tf.LayersModel) {
  const input = tf.tensor(games.map(({ game }) => getInputForGame(game)));

  const outputs = (network.predict(input) as tf.Tensor).dataSync();

  return games.map(({ game, whiteId, blackId }, i) => ({
    game: setGameResult(
      chooseMove(game, outputs.slice(i * 1972, (i + 1) * 1972))
    ),
    whiteId,
    blackId,
  }));
}

async function main() {
  const [, , generation, pairingBatch] = process.argv;
  if (!generation) {
    throw new Error("generation missing");
  }

  const pairingsFilename = path.join(
    __dirname,
    `generation${generation}`,
    `pairings_${pairingBatch}.txt`
  );
  const { pairings, move } = parsePairings(
    await fs.promises.readFile(pairingsFilename, "utf8")
  );

  if (Object.values(pairings).every((game) => game?.result)) {
    console.log("Already finished");
    return;
  }

  const networkIds = await fs.promises.readdir(
    path.join(__dirname, `generation${generation}`)
  );

  const networks = await networkIds
    .filter((id) => !id.match(/^pairings_\d+\.txt$/))
    .reduce<Promise<{ [id: string]: tf.LayersModel }>>(
      async (accPromise, id) => {
        const acc = await accPromise;
        acc[id] = await tf.loadLayersModel(
          `file://${path.resolve(
            __dirname,
            `generation${generation}`,
            id,
            "model.json"
          )}`
        );
        return acc;
      },
      Promise.resolve({})
    );

  const donePairings: Pairings = {};
  let gamesPerNetwork = Object.entries(pairings).reduce<GamesPerNetwork>(
    (acc, [id, game]) => {
      if (game?.result) {
        donePairings[id] = game;
      } else {
        const [whiteId, blackId] = id.split("-");
        acc[whiteId] = acc[whiteId] || [];
        acc[whiteId].push({
          game:
            game ||
            gameFromFen(
              "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
            ),
          whiteId,
          blackId,
        });
      }
      return acc;
    },
    {}
  );

  let newMove = 0;
  const doneGames: GameWithPlayers[] = [];
  while (
    Object.keys(gamesPerNetwork).length > 0 &&
    newMove < MOVES_PER_PLAY * 2
  ) {
    let halfMoveCounter = move + newMove++;
    console.log(
      `Move ${Math.floor(halfMoveCounter / 2 + 0.5)} ${
        halfMoveCounter % 2 === 0 ? "Black" : "White"
      }, ${Object.keys(gamesPerNetwork).length} networks still playing`
    );
    let newGamesPerNetwork: GamesPerNetwork = {};
    for (const networkId in gamesPerNetwork) {
      for (const gameWithPlayer of pickMoves(
        gamesPerNetwork[networkId],
        networks[networkId]
      )) {
        if (gameWithPlayer.game.result) {
          doneGames.push(gameWithPlayer);
        } else {
          const newNetworkId =
            gameWithPlayer.game.player === Player.WHITE
              ? gameWithPlayer.whiteId
              : gameWithPlayer.blackId;
          newGamesPerNetwork[newNetworkId] =
            newGamesPerNetwork[newNetworkId] || [];
          newGamesPerNetwork[newNetworkId].push(gameWithPlayer);
        }
      }
    }
    gamesPerNetwork = newGamesPerNetwork;
  }

  for (const { game, whiteId, blackId } of doneGames) {
    donePairings[`${whiteId}-${blackId}`] = game;
  }
  await fs.promises.writeFile(
    pairingsFilename,
    stringifyPairings({ pairings: donePairings, move: move + newMove })
  );

  for (const gameWithPlayerList of Object.values(gamesPerNetwork)) {
    const continuedPairings: Pairings = {};
    for (const { game, whiteId, blackId } of gameWithPlayerList) {
      continuedPairings[`${whiteId}-${blackId}`] = game;
    }
    await fs.promises.appendFile(
      pairingsFilename,
      stringifyPairings({ pairings: continuedPairings })
    );
  }

  throw new Error("not done yet");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
